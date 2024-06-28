# include .env file and export its env vars
# (-include to ignore error if it does not exist)
-include .env

.PHONY: RUN

run-debug-paraswap		:; ./target/debug/seamless-liquidator --rpc ${RPC_URL} --private-key ${PRIVATE_KEY} --bid-percentage 100 --deployment seamless --liquidator-address ${LIQUIDATOR_ADDRESS} --graphql-url ${GRAPHQL_URL} --dex-aggregator paraswap --minimum-debt-to-liquidate 500000000
run-release-paraswap	:; ./target/release/seamless-liquidator --rpc ${RPC_URL} --private-key ${PRIVATE_KEY} --bid-percentage 100 --deployment seamless --liquidator-address ${LIQUIDATOR_ADDRESS} --graphql-url ${GRAPHQL_URL} --dex-aggregator paraswap --minimum-debt-to-liquidate 500000000

run-debug		:; ./target/debug/seamless-liquidator --rpc ${RPC_URL} --private-key ${PRIVATE_KEY} --bid-percentage 100 --deployment seamless --liquidator-address ${LIQUIDATOR_ADDRESS} --graphql-url ${GRAPHQL_URL} --dex-aggregator none --minimum-debt-to-liquidate 0
run-release	:; ./target/release/seamless-liquidator --rpc ${RPC_URL} --private-key ${PRIVATE_KEY} --bid-percentage 100 --deployment seamless --liquidator-address ${LIQUIDATOR_ADDRESS} --graphql-url ${GRAPHQL_URL} --dex-aggregator none --minimum-debt-to-liquidate 0