use ethers::{
    prelude::abigen,
    providers::{Http, Provider},
    types::{Address, U256},
};
use eyre::Ok;
// use serde_json::Value;
use std::io;
use std::{convert::TryFrom, sync::Arc};
use tokio;

#[tokio::main]
#[allow(non_snake_case)]
async fn main() -> eyre::Result<()> {
    let INFURA_API_KEY = dotenvy::var("INFURA_API_KEY").expect("API KEY is need");
    let provider = Provider::try_from(format!("https://mainnet.infura.io/v3/{}", INFURA_API_KEY))?;
    let mut address_input = String::new();
    println!("Digite o endereço da carteira: ");
    io::stdin().read_line(&mut address_input)?;
    let address_from = address_input.trim().parse::<Address>()?;

    // Chame a função com o endereço

    print_balances(&provider, address_from).await?;

    Ok(())
}

async fn print_balances(provider: &Provider<Http>, address_from: Address) -> eyre::Result<()> {
    let token_address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse::<Address>()?;

    abigen!(
        IERC20,
        r#"[
            function totalSupply() external view returns (uint256)
            function balanceOf(address account) external view returns (uint256)
            function transfer(address recipient, uint256 amount) external returns (bool)
            function allowance(address owner, address spender) external view returns (uint256)
            function approve(address spender, uint256 amount) external returns (bool)
            function decimals() public view returns (uint8)
            function transferFrom( address sender, address recipient, uint256 amount) external returns (bool)
            event Transfer(address indexed from, address indexed to, uint256 value)
            event Approval(address indexed owner, address indexed spender, uint256 value)
        ]"#,
    );

    // Crie instância do contrato ERC20
    let client = Arc::new(provider);
    let contract = IERC20::new(token_address, client);

    // let base: f32 = 10.0;

    let result: U256 = (contract.balance_of(address_from).call()).await.unwrap();
    let decimal: u8 = (contract.decimals().call()).await.unwrap();
    let formatted_result = ethers::core::utils::format_units(result, decimal as u32)?;

    // let amount: f32 = result / base.powf(6.0);
    // if let Ok(balance_of) = contract.balance_of(address_from).call().await {
    //     println!("Total USDC is {balanceOf:?}");
    //     Err(balance_of);
    // }
    // println!(
    //     "Total USDC is {:?}",
    //     (contract.balance_of(address_from).call() / contract.decimals().call()).await
    // );
    // println!(
    //     "Total USDC : {:?}",
    //     formatted_result.unwrap().parse::<f64>()
    // );
    println!("Total USDC: {}", formatted_result);

    Ok(())
}
