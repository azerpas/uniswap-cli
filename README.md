# Script to DCA on Ethereum (on-chain)

DCA (Dollar Cost Averaging) is an investment strategy where you buy a fixed amount of a certain asset at regular intervals, regardless of the price. This way you can avoid buying at the top of a bull run and you can take advantage of the dips.

This script allows you to DCA on Ethereum (on-chain) by swapping a fixed amount of a certain token for another token. It uses ChaCha20Poly1305 encryption to encrypt your mnemonic and store it in a file. The wallet is then used to sign the transactions. It will ask you for a password to encrypt your mnemonic once and to unlock your wallet every time you run the script.

Use CRON to run this script at regular intervals.

## Installation
TODO

## Usage
TODO
```
Usage: dca-onchain [OPTIONS] --amount-to-swap <AMOUNT_TO_SWAP> --token-in <TOKEN_IN> --token-out <TOKEN_OUT>

Options:
  -u, --rpc <RPC>
          HTTP URL of the RPC endpoint to connect to [default: https://mainnet.infura.io/v3/]
  -n, --chain-id <CHAIN_ID>
          Chain ID of the network to connect to (1 for mainnet, 3 for ropsten, 4 for rinkeby, 5 for goerli, 42 for kovan, for more checkout https://chainlist.org/) [default: 1]
  -a, --amount-to-approve <AMOUNT_TO_APPROVE>
          Amount of tokens to approve for the swap. Make sure to use the correct decimals, e.g 1 USDT = 1000000 as USDT has 6 decimals. If set to 0 it will approve the amount of tokens required for the swap. If the flag is not specified, it will skip the approval step
  -s, --amount-to-swap <AMOUNT_TO_SWAP>
          Amount of tokens to swap. If you don't use the 'd' (decimals) flag, make sure to use the correct decimals: e.g 1 USDT = 1000000 as USDT has 6 decimals. If you want to swap 0.5 tokens, you need to specify 0.5 * 10^decimals For example, if you want to swap 0.5 USDT, you need to specify 500000 (6 decimals) If you want to swap 0.5 WETH, you need to specify 500000000000000000 (18 decimals)
  -d, --fetch-decimals <FETCH_DECIMALS>
          Get the decimals of the token you want to swap from. If used, the given `amount_to_swap` and `amount_to_approve` will be multiplied by 10^`decimals` to get the correct amount. `decimals` are fetched from the token contract [possible values: true, false]
  -i, --token-in <TOKEN_IN>
          Address of the token to swap from. This is the token you want to sell. It must be a valid ERC20 token address (e.g USDT: 0xdAC17F958D2ee523a2206206994597C13D831ec7)
  -o, --token-out <TOKEN_OUT>
          Address of the token to swap to. This is the token you want to buy. It must be a valid ERC20 token address (e.g WETH: 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2)
  -h, --help
          Print help
  -V, --version
          Print version
```