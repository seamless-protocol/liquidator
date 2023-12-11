// SPDX-License-Identifier: ISC
pragma solidity ^0.8.13;

interface IAugustus {
    function getTokenTransferProxy() external view returns (address);
}
