pub mod cli;
pub mod crypto;
pub mod discord;
pub mod explorer;
pub mod settings;
pub mod utils;
pub mod wallet;
use crate::{cli::Args, discord::WebhookMessage, wallet::decrypt_wallet_data};

use anyhow::{Context, Result};
use clap::Parser;
use ethers::{prelude::*, providers::Provider, signers::Signer};
use explorer::get_network_explorers;
use std::sync::Arc;

enum FeeAmount {
    LOWEST = 100,
    LOW = 500,
    MEDIUM = 3000,
    HIGH = 10000,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let wallet = decrypt_wallet_data()?;

    let provider = Provider::<Http>::try_from(args.rpc)?;
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(args.chain_id),
    ));

    const UNISWAP_V3_ROUTER_ADDRESS: &str = "0xE592427A0AEce92De3Edee1F18E0157C05861564";
    const UNISWAP_V3_QUOTER_ADDRESS: &str = "0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6";

    let router_address: Address = UNISWAP_V3_ROUTER_ADDRESS.parse()?;
    let quoter_address: Address = UNISWAP_V3_QUOTER_ADDRESS.parse()?;
    let token_in_address: Address = args.token_in.parse()?;
    let token_out_address: Address = args.token_out.parse()?;
    let network_explorers_map = get_network_explorers();
    let explorer = network_explorers_map
        .get(&(args.chain_id as u32))
        .or_else(|| {
            println!("No explorer found for chain id {}", args.chain_id);
            None
        });

    abigen!(UniswapV3Router, "./ISwapRouter.json");
    abigen!(
        UniswapV3Quoter,
        r#"[
            quoteExactInputSingle(address tokenIn, address tokenOut, uint24 fee, uint256 amountIn, uint160 sqrtPriceLimitX96) external returns (uint256 amountOut)
        ]"#r
    );
    abigen!(
        ERC20,
        r#"[
            approve(address spender, uint256 amount)(bool)
            decimals()(uint8)
            allowance(address owner, address spender)(uint256)
            symbol()(string)
        ]"#r
    );

    let router_contract = UniswapV3Router::new(router_address, client.clone());
    let quoter_contract = UniswapV3Quoter::new(quoter_address, client.clone());
    let token_in_contract = ERC20::new(token_in_address, client.clone());

    println!(
        "💫 Welcome to the Uniswap V3 Swap CLI 💫\n We will use wallet with address: {}",
        wallet.address()
    );

    // `decimals` is used to convert the amounts to the correct unit
    // by default it is set to 0, so the user can specify amounts in the token's smallest unit
    // e.g. for USDT this is 10^6, so the user can specify amounts in USDT's smallest unit (1e-6 USDT)
    // if using the `-d` flag, the decimals will be fetched from the token contract
    let mut decimals: u8 = 0;
    if args.fetch_decimals.is_some() {
        decimals = token_in_contract.decimals().call().await?;
        println!("Converting to {} decimals", decimals);
    }

    // Convert the amount to approve to the correct unit, based on the token's decimals
    let amount_to_swap = U256::from(args.amount_to_swap * 10u128.pow(decimals.into()));

    if args.amount_to_approve.is_some() {
        if args.amount_to_approve.unwrap() < args.amount_to_swap {
            println!("Amount to approve must be greater than amount to swap");
            return Ok(());
        }

        let amount_to_approve: U256;

        if args.amount_to_approve.unwrap() == 0 {
            if args.verbose {
                println!(
                    "Amount to approve not specified. Approving {} tokens.",
                    args.amount_to_swap
                );
            }
            amount_to_approve = amount_to_swap.clone();
        } else {
            if args.verbose {
                println!(
                    "Amount to approve specified. Approving {} tokens.",
                    args.amount_to_approve.unwrap()
                );
            }
            amount_to_approve =
                U256::from(args.amount_to_approve.unwrap() * 10u128.pow(decimals.into()));
        }

        // Approve the router to spend USDT
        let approve_tx = token_in_contract
            .approve(router_address, amount_to_approve);

        let approve_tx_gas = approve_tx.estimate_gas().await?;

        if args.verbose {
            println!("Approve transaction gas estimate: {}", approve_tx_gas);
        }

        let approve_tx: TransactionReceipt = approve_tx
            .gas(approve_tx_gas)
            .send()
            .await?
            .await?
            .expect("Approve transaction failed");

        println!(
            "Approved Uniswap V3 Router spending. Transaction Hash: {:?}",
            approve_tx.transaction_hash
        );
        if args.verbose {
            println!("Transaction Receipt: {:#?}", approve_tx);
        }
    } else {
        println!("Amount to approve not specified. Checking if allowance is sufficient.");
        let allowance: U256 = token_in_contract
            .allowance(wallet.address(), router_address)
            .call()
            .await?;
        if args.verbose {
            println!("Allowance is sufficient. Allowance: {}", allowance);
        }
        if allowance < amount_to_swap {
            println!("Allowance is less than amount to swap. Please use the `-a` flag to approve the router to spend '{}' tokens.", token_in_address);
            return Ok(());
        }
    }

    // Quote the swap
    let quote: U256 = quoter_contract
        .quote_exact_input_single(
            token_in_address,         // tokenIn: The token to be sold
            token_out_address,        // tokenOut: The token to be purchased
            FeeAmount::MEDIUM as u32, // fee: The fee level of the pool
            amount_to_swap,           // amountIn: The exact amount of tokenIn that will be swapped
            U256::from(0),            // sqrtPriceLimitX96: The optional price limit for the trade.
        )
        .call_raw()
        .await
        .with_context(|| {
            format!(
                "Failed to quote swap from {} to {}. Check that a pool exists for this pair.",
                args.token_in, args.token_out
            )
        })?;

    println!("Will swap {} for {} tokens\n", amount_to_swap, quote);

    let deadline = U256::from(chrono::Utc::now().timestamp() + 1200); // 20 minutes from now

    let swap_tx: TransactionReceipt = router_contract
        .exact_input_single(ExactInputSingleParams {
            token_in: token_in_address,
            token_out: token_out_address,
            fee: FeeAmount::MEDIUM as u32,
            recipient: wallet.address(),
            deadline: deadline,
            amount_in: amount_to_swap,
            amount_out_minimum: U256::from(quote),
            sqrt_price_limit_x96: U256::from(0),
        })
        .send()
        .await?
        .await?
        .expect("Swap transaction failed");

    println!(
        "🥳 Swap executed. Transaction Hash: {:?}",
        swap_tx.transaction_hash
    );
    if args.verbose {
        println!("Transaction Receipt: {:#?}", swap_tx);
    }
    if explorer.is_some() {
        println!(
            "View on explorer: {}/tx/{:?}",
            explorer.unwrap(),
            swap_tx.transaction_hash
        );
    }

    if args.webhook.is_some() {
        if args.verbose {
            println!("Sending webhook to {}", args.webhook.clone().unwrap());
            println!("Getting token_out decimals to convert amount_out to correct unit");
        }
        let token_out_contract = ERC20::new(token_out_address, client.clone());
        let token_out_decimals: u8 = token_out_contract.decimals().call().await?;

        let token_in_fmt = utils::format_token_amount(amount_to_swap.as_u64(), decimals.into(), 5);

        let token_out_fmt =
            utils::format_token_amount(quote.as_u64(), token_out_decimals.into(), 5);

        let token_in_symbol = match token_in_contract.symbol().call().await {
            Ok(s) => s,
            Err(_) => {
                println!("Failed to get token_in symbol");
                "tokens".to_string()
            }
        };

        let token_out_symbol = match token_out_contract.symbol().call().await {
            Ok(s) => s,
            Err(_) => {
                println!("Failed to get token_out symbol");
                "tokens".to_string()
            }
        };

        let message = format!(
            "You swapped {} {} for {} {}",
            token_in_fmt, token_in_symbol, token_out_fmt, token_out_symbol
        );

        match discord::send_message(&args.webhook.unwrap(), &WebhookMessage { content: message })
            .await
        {
            Ok(_) => println!("Webhook sent successfully"),
            Err(e) => println!("Failed to send webhook: {}", e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use ethers::{
        abi::Address,
        prelude::{abigen, rand::thread_rng, SignerMiddleware},
        providers::{Http, Provider},
        signers::{Signer, Wallet},
    };
    use std::env;
    use std::sync::Arc;

    #[tokio::test]
    async fn can_get_erc20_symbol() -> anyhow::Result<()> {
        abigen!(
            ERC20,
            r#"[
                symbol()(string)
            ]"#r
        );

        dotenv().ok();
        let rpc_url = env::var("RPC_URL").expect("RPC_URL not set");

        let provider = Provider::<Http>::try_from(rpc_url).unwrap();
        let wallet = Wallet::new(&mut thread_rng());
        let client = Arc::new(SignerMiddleware::new(
            provider,
            wallet.clone().with_chain_id(1u32),
        ));

        let usdt_contract = ERC20::new(
            "0xdAC17F958D2ee523a2206206994597C13D831ec7".parse::<Address>()?,
            client.clone(),
        );
        let usdc_contract = ERC20::new(
            "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse::<Address>()?,
            client.clone(),
        );
        let steth_contract = ERC20::new(
            "0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84".parse::<Address>()?,
            client.clone(),
        );

        let usdt_symbol = usdt_contract.symbol().call().await?;
        let usdc_symbol = usdc_contract.symbol().call().await?;
        let steth_symbol = steth_contract.symbol().call().await?;
        assert_eq!(usdt_symbol, "USDT");
        assert_eq!(usdc_symbol, "USDC");
        assert_eq!(steth_symbol, "stETH");
        Ok(())
    }
}
