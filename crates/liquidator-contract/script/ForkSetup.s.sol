// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import {IPool} from "../src/interfaces/IPool.sol";
import {MockAggregator} from "../test/util/MockAggregator.sol";
import {Constants} from "../test/util/Constants.sol";

contract ForkSetupScript is Script {
    bool constant DEPLOY_MOCK_AGGREGATOR = true;
    bool constant ENABLE_FLASH_LOANS = true;

    function getChainId() public view returns (uint256) {
        uint256 chainId;
        assembly {
            chainId := chainid()
        }
        return chainId;
    }

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployerAddress = vm.addr(deployerPrivateKey);

        console.log("Deployer address: ", deployerAddress);
        console.log("Deployer balance: ", deployerAddress.balance);
        console.log("BlockNumber: ", block.number);
        console.log("ChainId: ", getChainId());

        console.log("Deploying...");

        if (DEPLOY_MOCK_AGGREGATOR) {
            vm.startBroadcast(deployerPrivateKey);
            MockAggregator mockAggregator = new MockAggregator();

            console.log("Deployed MockAggregator to: ", address(mockAggregator));
            vm.stopBroadcast();
        }
        
        if (ENABLE_FLASH_LOANS) {
            vm.startBroadcast(Constants.SEAMLESS_ADMIN);

            address[] memory assets = IPool(address(Constants.POOL)).getReservesList();

            for (uint256 i = 0; i < assets.length; i++) {
                Constants.POOL_CONFIGURATOR.setReserveFlashLoaning(assets[i], true);
                console.log("Flash loan enabled for asset: ", assets[i]);
            }
            
            vm.stopBroadcast();
        }
    }
}
