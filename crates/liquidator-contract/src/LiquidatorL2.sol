// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Owned} from "solmate/auth/Owned.sol";
import {ERC20} from "solmate/tokens/ERC20.sol";
import {IL2Pool} from "./interfaces/IL2Pool.sol";
import {IUniswapV3SwapCallback} from "./interfaces/IUniswapV3SwapCallback.sol";
import {IUniswapV3PoolActions} from "./interfaces/IUniswapV3PoolActions.sol";
import {PoolAddress} from "./lib/PoolAddress.sol";
import {Path} from "./lib/Path.sol";

uint160 constant MIN_SQRT_RATIO = 4295128739;
/// @dev The maximum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MAX_TICK)
uint160 constant MAX_SQRT_RATIO = 1461446703485210103287273052203988822378723970342;

contract LiquidatorL2 is Owned(msg.sender), IUniswapV3SwapCallback {
    struct SwapCallbackData {
        bytes path;
        bytes32 liquidationArg1;
        bytes32 liquidationArg2;
    }

    address private constant uniswapV3Factory = 0x33128a8fC17869897dcE68Ed026d694621f6FDfD;
    IL2Pool public constant pool = IL2Pool(0x8F44Fd754285aa6A2b8B9B97739B79746e0475a7);

    constructor() {}

    /// @notice Performs a liquidation using a Uniswap v3 flash swap
    /// @param collateral address of the collateral asset to be liquidated
    /// @param debtToCover amount of debt asset to repay in exchange for collateral
    /// @param liquidationArg1 encoded argument for Aave L2 Pool liquidation call, see: IL2Pool and IL2Encoder
    /// @param liquidationArg2 encoded argument for Aave L2 Pool liquidation call, see: IL2Pool and IL2Encoder
    /// @param swapPath encoded path of pools to swap collateral through, see: https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps
    function liquidate(
        address collateral,
        uint256 debtToCover,
        bytes32 liquidationArg1,
        bytes32 liquidationArg2,
        bytes calldata swapPath
    ) external onlyOwner returns (int256 collateralGain) {
        uint256 collateralBalance = ERC20(collateral).balanceOf(address(this));

        swapOutUniswap(
            debtToCover,
            SwapCallbackData({path: swapPath, liquidationArg1: liquidationArg1, liquidationArg2: liquidationArg2})
        );

        collateralGain = int256(ERC20(collateral).balanceOf(address(this))) - int256(collateralBalance);
    }

    /// @inheritdoc IUniswapV3SwapCallback
    function uniswapV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata _data) external override {
        SwapCallbackData memory data = abi.decode(_data, (SwapCallbackData));

        (address tokenIn, address tokenOut, uint24 fee) = Path.decodeFirstPool(data.path);
        verifyCallback(uniswapV3Factory, PoolAddress.getPoolKey(tokenIn, tokenOut, fee));

        if (data.liquidationArg1 != "" && data.liquidationArg2 != "") {
            pool.liquidationCall(data.liquidationArg1, data.liquidationArg2);

            delete data.liquidationArg1;
            delete data.liquidationArg2;
        }

        uint256 amountToPay = amount0Delta > 0 ? uint256(amount0Delta) : uint256(amount1Delta);

        // either initiate the next swap or pay
        if (Path.hasMultiplePools(data.path)) {
            data.path = Path.skipToken(data.path);
            swapOutUniswap(amountToPay, data);
        }

        tokenIn = tokenOut; // swap in/out because exact output swaps are reversed
        ERC20(tokenIn).transfer(msg.sender, amountToPay);
    }

    /// @dev Performs a single exact output swap
    function swapOutUniswap(uint256 amountOut, SwapCallbackData memory data) internal {
        (address tokenOut, address tokenIn, uint24 fee) = Path.decodeFirstPool(data.path);

        bool zeroForOne = tokenIn < tokenOut;

        IUniswapV3PoolActions uniswapPool = IUniswapV3PoolActions(
            PoolAddress.computeAddress(uniswapV3Factory, PoolAddress.getPoolKey(tokenIn, tokenOut, fee))
        );

        uniswapPool.swap(
            address(this),
            zeroForOne,
            -int256(amountOut),
            zeroForOne ? MIN_SQRT_RATIO + 1 : MAX_SQRT_RATIO - 1,
            abi.encode(data)
        );
    }

    /// @notice Approve max ERC-20 allowance to Aave pool to save gas and not have to approve every liquidation
    /// @param token address of ERC-20 to approve
    function approvePool(address token) external onlyOwner {
        ERC20(token).approve(address(pool), type(uint256).max);
    }

    /// @notice Withdraw from contract (only owner)
    /// @param token address of ERC20 to withdraw, zero address withdraws native asset (i.e.: ETH)
    /// @param amount amount to withdraw
    function recover(address token, uint256 amount) external onlyOwner {
        if (token == address(0)) {
            payable(msg.sender).transfer(amount);
            return;
        }
        ERC20(token).transfer(msg.sender, amount);
    }

    /// @notice Returns the address of a valid Uniswap V3 Pool
    /// @param factory The contract address of the Uniswap V3 factory
    /// @param poolKey The identifying key of the V3 pool
    function verifyCallback(address factory, PoolAddress.PoolKey memory poolKey) internal view {
        address p = PoolAddress.computeAddress(factory, poolKey);
        require(msg.sender == p, "invalid pool");
    }
}
