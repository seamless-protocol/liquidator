// SPDX-License-Identifier: ISC
pragma solidity ^0.8.13;

interface IAugustusRegistry {
    function addAugustus(string calldata version, address augustus, bool isLatest) external;

    function banAugustus(address augustus) external;

    function isValidAugustus(address augustus) external view returns (bool);

    function getAugustusCount() external view returns (uint256);

    function getLatestVersion() external view returns (string memory);

    function getLatestAugustus() external view returns (address);

    function getAugustusByVersion(string calldata version) external view returns (address);
}
