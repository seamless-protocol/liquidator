// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {IPoolDataProvider} from "../../src/interfaces/IPoolDataProvider.sol";
import {IPoolConfigurator} from "../../src/interfaces/IPoolConfigurator.sol";
import {IL2Pool} from "../../src/interfaces/IL2Pool.sol";
import {IL2Encoder} from "../../src/interfaces/IL2Encoder.sol";
import {IPoolAddressesProvider} from "../../src/interfaces/IPoolAddressesProvider.sol";

library Constants {
    IERC20Metadata constant WETH = IERC20Metadata(0x4200000000000000000000000000000000000006);
    IERC20Metadata constant USDBC = IERC20Metadata(0xd9aAEc86B65D86f6A7B5B1b0c42FFA531710b6CA);
    IERC20Metadata constant CBETH = IERC20Metadata(0x2Ae3F1Ec7F1F5012CFEab0185bfc7aa3cf0DEc22);

    address constant SEAMLESS_ADMIN = 0xA1b5f2cc9B407177CD8a4ACF1699fa0b99955A22;

    IL2Pool constant POOL = IL2Pool(0x8F44Fd754285aa6A2b8B9B97739B79746e0475a7);
    IPoolDataProvider constant POOL_DATA_PROVIDER = IPoolDataProvider(0x2A0979257105834789bC6b9E1B00446DFbA8dFBa);
    IPoolAddressesProvider constant POOL_ADDRESSES_PROVIDER =
        IPoolAddressesProvider(0x0E02EB705be325407707662C6f6d3466E939f3a0);
    IPoolConfigurator constant POOL_CONFIGURATOR = IPoolConfigurator(0x7B08A77539A50218c8fB4B706B87fb799d3505A0);
    IL2Encoder constant ENCODER = IL2Encoder(0xceceF475167f7BFD8995c0cbB577644b623cD7Cf);

    address constant A_TOKEN_IMPL = 0x27076A995387458da63b23d9AFe3df851727A8dB;
    address constant STABLE_DEBT_TOKEN_IMPL = 0xb4D5e163738682A955404737f88FDCF15C1391bF;
    address constant VARIABLE_DEBT_TOKEN_IMPL = 0x3800DA378e17A5B8D07D0144c321163591475977;
    address constant INTEREST_RATE_STRATEGY_ADDRESS = 0x9610D3bA6576c2aC6Ffc75d02003cF811732aE33;
}
