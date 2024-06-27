// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IFlashLoan} from "./IFlashLoan.sol";

interface ILiquidatorParaswapL2 {
    error SenderNotPool(address);
    error InvalidInitiator(address);
    error InvalidAugustusInstance(address);
    error InvalidDebtBalance(uint256);

    struct FlashLoanCallbackData {
        uint256 debtBalanceBefore;
        IERC20 collateral;
        bytes32 liquidationArg1;
        bytes32 liquidationArg2;
        SwapParams swapParams;
    }

    struct LiquidationParams {
        // Address of the collateral to be liquidated
        IERC20 collateral;
        // Address of the debt to repay in exchange for collateral
        IERC20 debt;
        // Amount of debt to repay
        uint256 debtToCover;
        // Encoded argument for Aave L2 Pool liquidation call, see: IL2Pool and IL2Encoder
        bytes32 liquidationArg1;
        // Encoded argument for Aave L2 Pool liquidation call, see: IL2Pool and IL2Encoder
        bytes32 liquidationArg2;
    }

    struct SwapParams {
        address augustus;
        bytes swapCallData;
    }

    /// @notice Performs a liquidation using a flash loan and swapping collateral to debt using Paraswap
    /// @param liquidationParams Abi encoded data to perform liquidation
    /// @param flashLoanPool Address of pool to flash loan debtToCover from, see IFlashLoan
    /// @param swapParams Abi encoded data to execute swap on Paraswap
    /// @return collateralGain Amount of collateral gained by performing liquidation
    function liquidate(
        LiquidationParams calldata liquidationParams,
        IFlashLoan flashLoanPool,
        SwapParams calldata swapParams
    ) external returns (uint256 collateralGain, uint256 debtGain);
}
