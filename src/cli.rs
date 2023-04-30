use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// HTTP URL of the RPC endpoint to connect to
    #[arg(short = 'u', long, default_value_t = String::from("https://mainnet.infura.io/v3/"))]
    pub rpc: String,

    /// Chain ID of the network to connect to
    /// (1 for mainnet, 3 for ropsten, 4 for rinkeby, 5 for goerli, 42 for kovan, for more checkout https://chainlist.org/)
    #[arg(short = 'n', long, default_value_t = 1)]
    pub chain_id: u64,

    /// Amount of tokens to approve for the swap. Make sure to use the correct decimals, e.g 1 USDT = 1000000 as USDT has 6 decimals.
    /// If set to 0 it will approve the amount of tokens required for the swap.
    /// If the flag is not specified, it will skip the approval step.
    #[arg(short = 'a', long)]
    pub amount_to_approve: Option<u128>,

    /// Amount of tokens to swap. 
    /// If you don't use the 'd' (decimals) flag, make sure to use the correct decimals:
    ///     e.g 1 USDT = 1000000 as USDT has 6 decimals.
    ///     If you want to swap 0.5 tokens, you need to specify 0.5 * 10^decimals
    ///     For example, if you want to swap 0.5 USDT, you need to specify 500000 (6 decimals)
    ///     If you want to swap 0.5 WETH, you need to specify 500000000000000000 (18 decimals)
    #[arg(short = 's', long)]
    pub amount_to_swap: u128,

    /// Get the decimals of the token you want to swap from. 
    /// If used, the given `amount_to_swap` and `amount_to_approve` will be multiplied by 10^`decimals` to get the correct amount.
    /// `decimals` are fetched from the token contract.
    #[arg(short = 'd', long)]
    pub fetch_decimals: Option<bool>,

    /// Address of the token to swap from. This is the token you want to sell.
    /// It must be a valid ERC20 token address (e.g USDT: 0xdAC17F958D2ee523a2206206994597C13D831ec7)
    #[arg(short = 'i', long)]
    pub token_in: String,

    /// Address of the token to swap to. This is the token you want to buy.
    /// It must be a valid ERC20 token address (e.g WETH: 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2)
    #[arg(short = 'o', long)]
    pub token_out: String,

    /// Verbose mode, will print more information about the swap
    /// If not specified, it will only print the transaction hash
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Discord webhook URL to send the transaction hash to
    #[arg(short = 'w', long)]
    pub webhook: Option<String>,
}