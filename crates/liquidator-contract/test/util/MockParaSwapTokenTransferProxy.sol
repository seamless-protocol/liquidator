// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockParaSwapTokenTransferProxy is Test {
    uint256 toAmount;

    function setToAmount(uint256 amount) external {
        toAmount = amount;
    }

    function mockSwap(IERC20 from, IERC20 to, uint256 amount) external {
        from.transferFrom(msg.sender, address(this), amount);

        deal(address(to), msg.sender, to.balanceOf(msg.sender) + toAmount);
    }
}
