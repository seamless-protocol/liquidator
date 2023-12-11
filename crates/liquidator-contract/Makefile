# include .env file and export its env vars
# (-include to ignore error if it does not exist)
-include .env

.PHONY: test clean

# Build & test
build                   :; forge build
coverage                :; forge coverage
gas                     :; forge test --gas-report
gas-check               :; forge snapshot --check --tolerance 1
snapshot                :; forge snapshot
clean                   :; forge clean
fmt                     :; forge fmt
test                    :; forge test -vvvv --gas-report

# Deploy
deploy-liquidator			:; forge create ./src/Liquidator.sol:Liquidator --private-key ${PRIVATE_KEY} --rpc-url ${RPC_URL}
deploy-liquidator-paraswap	:; forge create ./src/LiquidatorParaswap.sol:LiquidatorParaswap --private-key ${PRIVATE_KEY} --rpc-url ${RPC_URL}
deploy-mock-aggregator		:; forge create ./test/util/MockAggregator.sol:MockAggregator --private-key ${PRIVATE_KEY} --rpc-url ${RPC_URL}

run-fork-setup				:; forge script ./script/ForkSetup.s.sol:ForkSetupScript --rpc-url ${RPC_URL} --slow --broadcast -vvvv --unlocked --sender 0x39E6b01214B15F6cc6339648511e3C91Ecd10dc7