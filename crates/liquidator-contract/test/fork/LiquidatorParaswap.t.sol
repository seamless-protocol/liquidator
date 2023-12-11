// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {IL2Pool} from "../../src/interfaces/IL2Pool.sol";
import {IPool} from "../../src/interfaces/IPool.sol";
import {IAugustusRegistry} from "../../src/interfaces/IAugustusRegistry.sol";
import {IAugustus} from "../../src/interfaces/IAugustus.sol";
import {Constants} from "../util/Constants.sol";
import {LiquidatorTestHelper} from "../util/LiquidatorTestHelper.sol";
import {MockParaSwapTokenTransferProxy} from "../util/MockParaSwapTokenTransferProxy.sol";
import {LiquidatorParaswap} from "../../src/LiquidatorParaswap.sol";
import {ILiquidatorParaswap} from "../../src/interfaces/ILiquidatorParaswap.sol";
import {IFlashLoan} from "../../src/interfaces/IFlashLoan.sol";

contract LiquidatorParaswapTest is LiquidatorTestHelper {
    LiquidatorParaswap liquidator;
    MockParaSwapTokenTransferProxy augustusAndTransferProxy;

    address augustusRegistry;

    function forkBlockNumber() internal pure override returns (uint256) {
        return 6561671;
    }

    function setUp() public override {
        super.setUp();

        liquidator = new LiquidatorParaswap();
        pool = liquidator.POOL();
        augustusRegistry = address(liquidator.AUGUSTUS_REGISTRY());
        augustusAndTransferProxy = new MockParaSwapTokenTransferProxy();

        vm.startPrank(Constants.SEAMLESS_ADMIN);

        enableFlashLoan(address(Constants.WETH));
        enableFlashLoan(address(Constants.USDBC));
        enableFlashLoan(address(Constants.CBETH));

        vm.stopPrank();
    }

    function test_RevertIfNot_FlashPool() public {
        vm.expectRevert(abi.encodeWithSelector(ILiquidatorParaswap.SenderNotPool.selector, address(this)));
        liquidator.executeOperation(new address[](0), new uint256[](0), new uint256[](0), address(this), abi.encode());
    }

    function test_RevertIfNot_SelfInitiator() public {
        vm.startPrank(address(0));
        vm.expectRevert(abi.encodeWithSelector(ILiquidatorParaswap.InvalidInitiator.selector, address(this)));
        liquidator.executeOperation(new address[](0), new uint256[](0), new uint256[](0), address(this), abi.encode());
        vm.stopPrank();
    }

    function test_RevertIfNot_ValidAugustus() public {
        (uint256 debtToCover,,) = setupLiquidatablePosition(
            SetupLiquidationPositionParams(
                Constants.WETH,
                Constants.USDBC,
                1 ether,
                1200 * 10 ** 6,
                1300 ether,
                10 ** Constants.WETH.decimals(),
                10 ** Constants.USDBC.decimals()
            )
        );

        vm.mockCall(
            augustusRegistry,
            abi.encodeWithSelector(IAugustusRegistry.isValidAugustus.selector, address(augustusAndTransferProxy)),
            abi.encode(false)
        );

        ILiquidatorParaswap.SwapParams memory swapParams =
            ILiquidatorParaswap.SwapParams({augustus: address(augustusAndTransferProxy), swapCallData: abi.encode()});

        (bytes32 arg1, bytes32 arg2) = Constants.ENCODER.encodeLiquidationCall(
            address(Constants.WETH), address(Constants.USDBC), user, debtToCover, false
        );

        vm.expectRevert(
            abi.encodeWithSelector(
                ILiquidatorParaswap.InvalidAugustusInstance.selector, address(augustusAndTransferProxy)
            )
        );

        liquidator.liquidate(
            ILiquidatorParaswap.LiquidationParams({
                collateral: Constants.WETH,
                debt: Constants.USDBC,
                debtToCover: debtToCover,
                liquidationArg1: arg1,
                liquidationArg2: arg2
            }),
            IFlashLoan(address(pool)),
            swapParams
        );
    }

    function testFuzz_LiquidationWethCollateral(uint256 profit) public {
        profit = bound(profit, 0, 1 ether);

        (uint256 debtToCover, uint256 collateralToLiquidate,) = setupLiquidatablePosition(
            SetupLiquidationPositionParams(
                Constants.WETH,
                Constants.USDBC,
                1 ether,
                1200 * 10 ** 6,
                1300 ether,
                10 ** Constants.WETH.decimals(),
                10 ** Constants.USDBC.decimals()
            )
        );

        (, uint256 totalDebt,,,, uint256 healthFactor) = pool.getUserAccountData(user);
        console.log("totalDebt: ", totalDebt);
        console.log("healthFactor: ", healthFactor);
        console.log("debtToCover: ", debtToCover);

        vm.mockCall(
            augustusRegistry,
            abi.encodeWithSelector(IAugustusRegistry.isValidAugustus.selector, address(augustusAndTransferProxy)),
            abi.encode(true)
        );
        vm.mockCall(
            address(augustusAndTransferProxy),
            abi.encodeWithSelector(IAugustus.getTokenTransferProxy.selector),
            abi.encode(augustusAndTransferProxy)
        );

        uint256 repayAmount =
            profit + debtToCover + percentMul(debtToCover, IPool(address(pool)).FLASHLOAN_PREMIUM_TOTAL());
        augustusAndTransferProxy.setToAmount(repayAmount);

        ILiquidatorParaswap.SwapParams memory swapParams = ILiquidatorParaswap.SwapParams({
            augustus: address(augustusAndTransferProxy),
            swapCallData: abi.encodeWithSelector(
                MockParaSwapTokenTransferProxy.mockSwap.selector, Constants.WETH, Constants.USDBC, collateralToLiquidate
                )
        });

        (bytes32 arg1, bytes32 arg2) = Constants.ENCODER.encodeLiquidationCall(
            address(Constants.WETH), address(Constants.USDBC), user, debtToCover, false
        );
        liquidator.liquidate(
            ILiquidatorParaswap.LiquidationParams({
                collateral: Constants.WETH,
                debt: Constants.USDBC,
                debtToCover: debtToCover,
                liquidationArg1: arg1,
                liquidationArg2: arg2
            }),
            IFlashLoan(address(pool)),
            swapParams
        );

        assertEq(Constants.USDBC.balanceOf(address(liquidator)), profit);
    }

    // TODO: test swap debt profit
    // TODO: test swap collateral profit
}
