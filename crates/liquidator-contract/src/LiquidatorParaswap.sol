// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {IPool} from "./interfaces/IPool.sol";
import {IFlashLoanReceiver} from "./interfaces/IFlashLoanReceiver.sol";
import {IFlashLoan} from "./interfaces/IFlashLoan.sol";
import {IAugustusRegistry} from "./interfaces/IAugustusRegistry.sol";
import {IAugustus} from "./interfaces/IAugustus.sol";
import {ILiquidatorParaswap} from "./interfaces/ILiquidatorParaswap.sol";

contract LiquidatorParaswap is ILiquidatorParaswap, IFlashLoanReceiver, Ownable {
    IPool public constant POOL = IPool(0x8F44Fd754285aa6A2b8B9B97739B79746e0475a7);
    IAugustusRegistry public constant AUGUSTUS_REGISTRY = IAugustusRegistry(0x7E31B336F9E8bA52ba3c4ac861b033Ba90900bb3);

    IFlashLoan private _flashPool;

    constructor() Ownable(msg.sender) {}

    /// @inheritdoc ILiquidatorParaswap
    function liquidate(
        LiquidationParams calldata liquidationParams,
        IFlashLoan flashLoanPool,
        SwapParams calldata swapParams
    ) external onlyOwner returns (uint256 collateralGain, uint256 debtGain) {
        uint256 debtBalanceBefore = liquidationParams.debt.balanceOf(address(this));
        uint256 collateralBalanceBefore = liquidationParams.collateral.balanceOf(address(this));

        _flashPool = flashLoanPool;

        // Don't use flashLoanSimple in case governance wants to allowlist liquidators from flash loan fee
        IFlashLoan(_flashPool).flashLoan(
            address(this),
            _convertAddressToDynamicArray(address(liquidationParams.debt)),
            _convertUint256ToDynamicArray(liquidationParams.debtToCover),
            _convertUint256ToDynamicArray(0),
            address(this),
            abi.encode(
                FlashLoanCallbackData({
                    debtBalanceBefore: debtBalanceBefore,
                    collateral: liquidationParams.collateral,
                    liquidationCallData: liquidationParams.liquidationCallData,
                    swapParams: swapParams
                })
            ),
            0
        );

        delete _flashPool;

        collateralGain = liquidationParams.collateral.balanceOf(address(this)) - collateralBalanceBefore;
        debtGain = liquidationParams.debt.balanceOf(address(this)) - debtBalanceBefore;
    }

    /// @inheritdoc IFlashLoanReceiver
    function executeOperation(
        address[] calldata assets,
        uint256[] calldata amounts,
        uint256[] calldata premiums,
        address initiator,
        bytes calldata _params
    ) external returns (bool success) {
        if (msg.sender != address(_flashPool)) {
            revert SenderNotPool(msg.sender);
        }
        if (initiator != address(this)) {
            revert InvalidInitiator(initiator);
        }

        FlashLoanCallbackData memory params = abi.decode(_params, (FlashLoanCallbackData));

        _liquidateAndSwap(
            IAugustus(params.swapParams.augustus),
            IERC20(assets[0]),
            params.collateral,
            amounts[0],
            params.liquidationCallData,
            params.swapParams.swapCallData
        );

        // Approve lending pool so flash loan can be repaid
        uint256 repayAmount = amounts[0] + premiums[0];

        uint256 finalDebtBalance = IERC20(assets[0]).balanceOf(address(this)) - repayAmount;
        if (finalDebtBalance < params.debtBalanceBefore) {
            revert InvalidDebtBalance(finalDebtBalance);
        }

        SafeERC20.forceApprove(IERC20(assets[0]), address(_flashPool), repayAmount);

        return true;
    }

    /// @dev liquidate a position on seamless and swap the collateral obtained back into the debt asset repaid
    /// @param augustus Address of Paraswap Augustus contract to use for swap
    /// @param debt Debt ERC20 token that is used to repay the liquidatable debt
    /// @param collateral Collateral ERC20 token received for liquidating position
    /// @param debtToCover The amount of debt to repay
    /// @param liquidationCallData Calldata for liquidationCall
    /// @param swapCallData Abi encoded data to execute swap on Paraswap
    function _liquidateAndSwap(
        IAugustus augustus,
        IERC20 debt,
        IERC20 collateral,
        uint256 debtToCover,
        bytes memory liquidationCallData,
        bytes memory swapCallData
    ) internal {
        uint256 collateralBalanceBefore = collateral.balanceOf(address(this));

        SafeERC20.forceApprove(debt, address(POOL), debtToCover);
        (bool success,) = address(POOL).call(liquidationCallData);
        if (!success) {
            revert FailedLiquidationCall();
        }

        uint256 collateralReceived = collateral.balanceOf(address(this)) - collateralBalanceBefore;

        if (!AUGUSTUS_REGISTRY.isValidAugustus(address(augustus))) {
            revert InvalidAugustusInstance(address(augustus));
        }

        address tokenTransferProxy = augustus.getTokenTransferProxy();

        SafeERC20.forceApprove(collateral, tokenTransferProxy, collateralReceived);

        // Reverts if function call does not succeed
        Address.functionCall(address(augustus), swapCallData);

        // Reset approve after swap in case full allowance not used
        SafeERC20.forceApprove(collateral, tokenTransferProxy, 0);
    }

    /// @dev Helper to convert single address to a dynamic array of length 1
    /// @param address_ Uint to convert
    /// @return ret Dynamic memory array of length 1
    function _convertAddressToDynamicArray(address address_) internal pure returns (address[] memory ret) {
        ret = new address[](1);
        ret[0] = address_;
    }

    /// @dev Helper to convert single uint256 to a dynamic array of length 1
    /// @param uint256_ Uint to convert
    /// @return ret Dynamic memory array of length 1
    function _convertUint256ToDynamicArray(uint256 uint256_) internal pure returns (uint256[] memory ret) {
        ret = new uint256[](1);
        ret[0] = uint256_;
    }
}
