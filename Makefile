# include .env file and export its env vars
# (-include to ignore error if it does not exist)
-include .env

.PHONY: RUN

run-debug-paraswap		:; ./target/debug/seamless-liquidator --rpc ${RPC_URL} --private-key ${PRIVATE_KEY} --bid-percentage 100 --deployment seashell --liquidator-address ${LIQUIDATOR_ADDRESS} --dex-aggregator paraswap
run-release-paraswap	:; ./target/release/seamless-liquidator --rpc ${RPC_URL} --private-key ${PRIVATE_KEY} --bid-percentage 100 --deployment seashell --liquidator-address ${LIQUIDATOR_ADDRESS} --dex-aggregator paraswap

run-debug		:; ./target/debug/seamless-liquidator --rpc ${RPC_URL} --private-key ${PRIVATE_KEY} --bid-percentage 100 --deployment seashell --liquidator-address ${LIQUIDATOR_ADDRESS}
run-release	:; ./target/release/seamless-liquidator --rpc ${RPC_URL} --private-key ${PRIVATE_KEY} --bid-percentage 100 --deployment seashell --liquidator-address ${LIQUIDATOR_ADDRESS}