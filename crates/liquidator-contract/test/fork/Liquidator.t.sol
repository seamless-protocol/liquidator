// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC20} from "solmate/tokens/ERC20.sol";
import {Test, console2} from "forge-std/Test.sol";
import {MockOracle} from "../util/MockOracle.sol";
import {Liquidator} from "../../src/Liquidator.sol";
import {IL2Pool} from "../../src/interfaces/IL2Pool.sol";
import {IL2Encoder} from "../../src/interfaces/IL2Encoder.sol";
import {IAaveOracle} from "../../src/interfaces/IAaveOracle.sol";
import {IPoolAddressesProvider} from "../../src/interfaces/IAddressesProvider.sol";
import {IPoolDataProvider} from "../../src/interfaces/IPoolDataProvider.sol";
import {IPoolConfigurator} from "../../src/interfaces/IPoolConfigurator.sol";
import {ConfiguratorInputTypes} from "../../src/interfaces/ConfiguratorInputTypes.sol";
import {IQuoterV2} from "../../src/interfaces/IQuoterV2.sol";

contract LiquidatorTest is Test {
    ERC20 constant weth = ERC20(0x4200000000000000000000000000000000000006);
    ERC20 constant usdc = ERC20(0xd9aAEc86B65D86f6A7B5B1b0c42FFA531710b6CA);
    ERC20 constant cbeth = ERC20(0x2Ae3F1Ec7F1F5012CFEab0185bfc7aa3cf0DEc22);
    IQuoterV2 constant quoter = IQuoterV2(0x3d4e44Eb1374240CE5F1B871ab261CD16335B76a);
    IL2Encoder constant encoder = IL2Encoder(0xceceF475167f7BFD8995c0cbB577644b623cD7Cf);
    // IAaveOracle constant oracle = IAaveOracle(0x2Cc0Fc26eD4563A5ce5e8bdcfe1A2878676Ae156);
    IPoolDataProvider constant dataProvider = IPoolDataProvider(0x2A0979257105834789bC6b9E1B00446DFbA8dFBa);
    MockOracle oracle;
    IPoolAddressesProvider constant addressesProvider =
        IPoolAddressesProvider(0x0E02EB705be325407707662C6f6d3466E939f3a0);
    IPoolConfigurator poolConfigurator = IPoolConfigurator(0x7B08A77539A50218c8fB4B706B87fb799d3505A0);
    address constant aaveAdmin = 0xA1b5f2cc9B407177CD8a4ACF1699fa0b99955A22;
    uint256 constant wethUnit = 10 ** 18;
    uint256 constant usdcUnit = 10 ** 6;
    Liquidator liquidator;
    IL2Pool pool;
    address user;

    function setUp() public {
        vm.createSelectFork(vm.envString("FORK_URL"));
        oracle = new MockOracle();
        user = makeAddr("user");
        liquidator = new Liquidator();
        liquidator.approvePool(address(weth));
        liquidator.approvePool(address(usdc));
        liquidator.approvePool(address(cbeth));
        pool = liquidator.pool();
        deal(address(weth), user, 100 ether);
        deal(address(cbeth), user, 100 ether);
        deal(address(usdc), user, 10000 * 10 ** 6);

        oracle.setAssetPrice(address(weth), 1700 ether);
        oracle.setAssetPrice(address(usdc), 1 ether);
        oracle.setAssetPrice(address(cbeth), 1700 ether);
        oracle.setEthUsdPrice(1700 ether);
        vm.startPrank(aaveAdmin);

        (address aToken,,) = dataProvider.getReserveTokensAddresses(address(cbeth));
        if (aToken == address(0)) {
            addAaveMarket(cbeth);
        }

        addressesProvider.setPriceOracle(address(oracle));
        vm.stopPrank();
    }

    function testLiquidationWethCollateral() public {
        (uint256 debtToCover, uint256 expectedLiquidationReward) = setupLiquidatablePosition(
            SetupLiquidationPositionParams(weth, usdc, 1 ether, 1200 * 10 ** 6, 1200 ether, wethUnit, usdcUnit)
        );
        uint24 poolFee = 500;
        (uint256 wethRequired,,,) = quoter.quoteExactOutputSingle(
            IQuoterV2.QuoteExactOutputSingleParams({
                tokenIn: address(weth),
                tokenOut: address(usdc),
                amount: debtToCover,
                fee: poolFee,
                sqrtPriceLimitX96: 4295128740
            })
        );
        uint256 expectedGain = expectedLiquidationReward - wethRequired;

        (bytes32 arg1, bytes32 arg2) =
            encoder.encodeLiquidationCall(address(weth), address(usdc), user, debtToCover, false);
        liquidator.liquidate(
            address(weth), debtToCover, arg1, arg2, abi.encodePacked(address(usdc), poolFee, address(weth))
        );
        assertEq(weth.balanceOf(address(liquidator)), expectedGain);
    }

    function testLiquidationUsdcCollateral() public {
        (uint256 debtToCover, uint256 expectedLiquidationReward) = setupLiquidatablePosition(
            SetupLiquidationPositionParams(usdc, weth, 2500 * 10 ** 6, 1 ether, 0.5 ether, usdcUnit, wethUnit)
        );

        uint24 poolFee = 500;
        (uint256 usdcRequired,,,) = quoter.quoteExactOutputSingle(
            IQuoterV2.QuoteExactOutputSingleParams({
                tokenIn: address(usdc),
                tokenOut: address(weth),
                amount: debtToCover,
                fee: poolFee,
                sqrtPriceLimitX96: 1461446703485210103287273052203988822378723970341
            })
        );
        uint256 expectedGain = expectedLiquidationReward - usdcRequired;

        (bytes32 arg1, bytes32 arg2) =
            encoder.encodeLiquidationCall(address(usdc), address(weth), user, debtToCover, false);
        liquidator.liquidate(
            address(usdc), debtToCover, arg1, arg2, abi.encodePacked(address(weth), poolFee, address(usdc))
        );
        assertEq(usdc.balanceOf(address(liquidator)), expectedGain);
    }

    // No Uniswap v3 pair for cbETH/USDbC. We liquidate using a multi-hop swap going through WETH/USDbC pool
    function testLiquidationCbethCollateral() public {
        (uint256 debtToCover, uint256 expectedLiquidationReward) = setupLiquidatablePosition(
            SetupLiquidationPositionParams(cbeth, usdc, 1 ether, 1200 * 10 ** 6, 1200 ether, wethUnit, usdcUnit)
        );
        uint24 poolFee = 500;
        bytes memory swapPath = abi.encodePacked(address(usdc), poolFee, address(weth), poolFee, address(cbeth));
        (uint256 cbethRequired,,,) = quoter.quoteExactOutput(swapPath, debtToCover);
        uint256 expectedGain = expectedLiquidationReward - cbethRequired;

        (bytes32 arg1, bytes32 arg2) =
            encoder.encodeLiquidationCall(address(cbeth), address(usdc), user, debtToCover, false);
        liquidator.liquidate(address(cbeth), debtToCover, arg1, arg2, swapPath);
        assertEq(cbeth.balanceOf(address(liquidator)), expectedGain);
    }

    struct SetupLiquidationPositionParams {
        ERC20 collateral;
        ERC20 debt;
        uint256 collateralAmount;
        uint256 debtAmount;
        uint256 collateralAssetPrice;
        uint256 collateralUnit;
        uint256 debtUnit;
    }

    function setupLiquidatablePosition(SetupLiquidationPositionParams memory params)
        internal
        returns (uint256 debtToCover, uint256 expectedLiquidationReward)
    {
        vm.startPrank(user);
        params.collateral.approve(address(pool), type(uint256).max);
        pool.supply(encoder.encodeSupplyParams(address(params.collateral), params.collateralAmount, 0));
        pool.borrow(encoder.encodeBorrowParams(address(params.debt), params.debtAmount, 2, 0));
        vm.stopPrank();
        uint256 debtAssetPrice = oracle.getAssetPrice(address(params.debt));
        oracle.setAssetPrice(address(params.collateral), params.collateralAssetPrice);
        (address collateralAToken,,) = dataProvider.getReserveTokensAddresses(address(params.collateral));
        uint256 userCollateralBalance = ERC20(collateralAToken).balanceOf(user);
        // user now has a liquidatable position with a 100% close factor

        (,,, uint256 bonus,,,,,,) = dataProvider.getReserveConfigurationData(address(params.collateral));
        uint256 protocolFee = dataProvider.getLiquidationProtocolFee(address(params.collateral));
        (, uint256 currentStableDebt, uint256 currentVariableDebt,,,,,,) =
            dataProvider.getUserReserveData(address(params.debt), user);
        debtToCover = currentStableDebt + currentVariableDebt;

        uint256 baseCollateral =
            (debtAssetPrice * debtToCover * params.collateralUnit) / (params.collateralAssetPrice * params.debtUnit);
        uint256 collateralToLiquidate = percentMul(baseCollateral, bonus);

        if (collateralToLiquidate > userCollateralBalance) {
            collateralToLiquidate = userCollateralBalance;
            debtToCover = (params.collateralAssetPrice * collateralToLiquidate * params.debtUnit)
                / percentDiv((debtAssetPrice * params.collateralUnit), bonus);
        }

        uint256 bonusCollateral = collateralToLiquidate - percentDiv(collateralToLiquidate, bonus);

        uint256 liquidationProtocolFees = percentMul(bonusCollateral, protocolFee);
        expectedLiquidationReward = collateralToLiquidate - liquidationProtocolFees;
    }

    function addAaveMarket(ERC20 asset) internal {
        string memory symbol = asset.symbol();

        ConfiguratorInputTypes.InitReserveInput[] memory reserves = new ConfiguratorInputTypes.InitReserveInput[](1);

        reserves[0] = ConfiguratorInputTypes.InitReserveInput({
            aTokenImpl: 0x27076A995387458da63b23d9AFe3df851727A8dB,
            stableDebtTokenImpl: 0xb4D5e163738682A955404737f88FDCF15C1391bF,
            variableDebtTokenImpl: 0x3800DA378e17A5B8D07D0144c321163591475977,
            underlyingAssetDecimals: asset.decimals(),
            interestRateStrategyAddress: 0x9610D3bA6576c2aC6Ffc75d02003cF811732aE33,
            underlyingAsset: address(asset),
            treasury: 0x982F3A0e3183896f9970b8A9Ea6B69Cd53AF1089,
            incentivesController: 0x91Ac2FfF8CBeF5859eAA6DdA661feBd533cD3780,
            aTokenName: string.concat("Seamless ", symbol),
            aTokenSymbol: string.concat("s", symbol),
            variableDebtTokenName: string.concat("Seamless Variable Debt ", symbol),
            variableDebtTokenSymbol: string.concat("variableDebtSeam", symbol),
            stableDebtTokenName: string.concat("Seamless Stable Debt ", symbol),
            stableDebtTokenSymbol: string.concat("stableDebtSeam", symbol),
            params: abi.encode()
        });

        poolConfigurator.initReserves(reserves);

        poolConfigurator.configureReserveAsCollateral(address(asset), 7500, 8000, 10500);

        poolConfigurator.setReserveBorrowing(address(asset), true);
    }

    function percentMul(uint256 a, uint256 bps) internal pure returns (uint256) {
        return (5000 + (a * bps)) / 10000;
    }

    function percentDiv(uint256 a, uint256 bps) internal pure returns (uint256) {
        uint256 halfBps = (bps / 2);
        return (halfBps + (a * 10000)) / bps;
    }
}
