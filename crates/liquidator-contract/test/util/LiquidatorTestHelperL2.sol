// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {MockOracle} from "../util/MockOracle.sol";
import {IL2Pool} from "../../src/interfaces/IL2Pool.sol";
import {ConfiguratorInputTypes} from "../../src/interfaces/ConfiguratorInputTypes.sol";
import {Constants} from "./Constants.sol";

abstract contract LiquidatorTestHelperL2 is Test {
    struct SetupLiquidationPositionParams {
        IERC20Metadata collateral;
        IERC20Metadata debt;
        uint256 collateralAmount;
        uint256 debtAmount;
        uint256 collateralAssetPrice;
        uint256 collateralUnit;
        uint256 debtUnit;
    }

    MockOracle oracle;
    address user;
    IL2Pool pool;

    function forkBlockNumber() internal pure virtual returns (uint256) {
        return 4878098;
    }

    function setUp() public virtual {
        vm.createSelectFork(vm.envString("FORK_URL"), forkBlockNumber());
        oracle = new MockOracle();
        user = makeAddr("user");

        deal(address(Constants.WETH), user, 100 ether);
        deal(address(Constants.CBETH), user, 100 ether);
        deal(address(Constants.USDBC), user, 10000 * 10 ** 6);

        oracle.setAssetPrice(address(Constants.WETH), 1700 ether);
        oracle.setAssetPrice(address(Constants.USDBC), 1 ether);
        oracle.setAssetPrice(address(Constants.CBETH), 1700 ether);
        oracle.setEthUsdPrice(1700 ether);
        vm.startPrank(Constants.SEAMLESS_ADMIN);

        (address aToken,,) = Constants.POOL_DATA_PROVIDER.getReserveTokensAddresses(address(Constants.CBETH));
        if (aToken == address(0)) {
            addAaveMarket(Constants.CBETH);
        }

        Constants.POOL_ADDRESSES_PROVIDER.setPriceOracle(address(oracle));
        vm.stopPrank();
    }

    function addAaveMarket(IERC20Metadata asset) internal {
        string memory symbol = asset.symbol();

        ConfiguratorInputTypes.InitReserveInput[] memory reserves = new ConfiguratorInputTypes.InitReserveInput[](1);

        reserves[0] = ConfiguratorInputTypes.InitReserveInput({
            aTokenImpl: Constants.A_TOKEN_IMPL,
            stableDebtTokenImpl: Constants.STABLE_DEBT_TOKEN_IMPL,
            variableDebtTokenImpl: Constants.VARIABLE_DEBT_TOKEN_IMPL,
            underlyingAssetDecimals: asset.decimals(),
            interestRateStrategyAddress: Constants.INTEREST_RATE_STRATEGY_ADDRESS,
            underlyingAsset: address(asset),
            treasury: address(0),
            incentivesController: address(0),
            aTokenName: string.concat("Seamless ", symbol),
            aTokenSymbol: string.concat("s", symbol),
            variableDebtTokenName: string.concat("Seamless Variable Debt ", symbol),
            variableDebtTokenSymbol: string.concat("variableDebtSeam", symbol),
            stableDebtTokenName: string.concat("Seamless Stable Debt ", symbol),
            stableDebtTokenSymbol: string.concat("stableDebtSeam", symbol),
            params: abi.encode()
        });

        Constants.POOL_CONFIGURATOR.initReserves(reserves);

        Constants.POOL_CONFIGURATOR.configureReserveAsCollateral(address(asset), 7500, 8000, 10500);

        Constants.POOL_CONFIGURATOR.setReserveBorrowing(address(asset), true);
    }

    function enableFlashLoan(address asset) internal {
        Constants.POOL_CONFIGURATOR.setReserveFlashLoaning(asset, true);
    }

    function setupLiquidatablePosition(SetupLiquidationPositionParams memory params)
        internal
        returns (uint256 debtToCover, uint256 collateralToLiquidate, uint256 expectedLiquidationReward)
    {
        vm.startPrank(user);
        params.collateral.approve(address(pool), type(uint256).max);
        pool.supply(Constants.ENCODER.encodeSupplyParams(address(params.collateral), params.collateralAmount, 0));
        pool.borrow(Constants.ENCODER.encodeBorrowParams(address(params.debt), params.debtAmount, 2, 0));
        vm.stopPrank();
        uint256 debtAssetPrice = oracle.getAssetPrice(address(params.debt));
        oracle.setAssetPrice(address(params.collateral), params.collateralAssetPrice);
        (address collateralAToken,,) =
            Constants.POOL_DATA_PROVIDER.getReserveTokensAddresses(address(params.collateral));
        uint256 userCollateralBalance = IERC20Metadata(collateralAToken).balanceOf(user);
        // user now has a liquidatable position with a 100% close factor

        (,,, uint256 bonus,,,,,,) = Constants.POOL_DATA_PROVIDER.getReserveConfigurationData(address(params.collateral));
        uint256 protocolFee = Constants.POOL_DATA_PROVIDER.getLiquidationProtocolFee(address(params.collateral));
        (, uint256 currentStableDebt, uint256 currentVariableDebt,,,,,,) =
            Constants.POOL_DATA_PROVIDER.getUserReserveData(address(params.debt), user);
        debtToCover = currentStableDebt + currentVariableDebt;

        uint256 baseCollateral =
            (debtAssetPrice * debtToCover * params.collateralUnit) / (params.collateralAssetPrice * params.debtUnit);
        collateralToLiquidate = percentMul(baseCollateral, bonus);

        if (collateralToLiquidate > userCollateralBalance) {
            collateralToLiquidate = userCollateralBalance;
            debtToCover = (params.collateralAssetPrice * collateralToLiquidate * params.debtUnit)
                / percentDiv((debtAssetPrice * params.collateralUnit), bonus);
        }

        uint256 bonusCollateral = collateralToLiquidate - percentDiv(collateralToLiquidate, bonus);

        uint256 liquidationProtocolFees = percentMul(bonusCollateral, protocolFee);
        expectedLiquidationReward = collateralToLiquidate - liquidationProtocolFees;
    }

    function percentMul(uint256 a, uint256 bps) internal pure returns (uint256) {
        return (5000 + (a * bps)) / 10000;
    }

    function percentDiv(uint256 a, uint256 bps) internal pure returns (uint256) {
        uint256 halfBps = (bps / 2);
        return (halfBps + (a * 10000)) / bps;
    }
}
