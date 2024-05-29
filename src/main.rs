use ethers::{
    prelude::abigen,
    providers::{Http, Provider},
    types::{Address, U256},
};
use eyre::Ok;
use std::io;
use std::{convert::TryFrom, sync::Arc};
use tokio;

// todo
/*  > Handle errors;
   Ok Handle 0 coins in wallet;
   Ok Handle wrong address pattern - if user input a contract address return - "This is not a valid wallet address"
   > Multichain;
   Ok User can be able to change the chain(chainId) for those: Ethereum, Polygon, Optimism
   > UX CLI:
   Ok pretty formatting;
   > change output print;
   > Using up-to-date libs;
   - This is needed because ethers.rs has been deprecated

*/

#[derive(Debug)]
enum Network {
    Ethereum(u8, Address),
    Polygon(u8, Address),
    Optimism(u8, Address),
}

#[tokio::main]
#[allow(non_snake_case)]
async fn main() -> eyre::Result<()> {
    // Get user input for network selection
    println!("Qual a rede blockchain desejada?");
    println!("a - Ethereum;");
    println!("b - Polygon;");
    println!("c - Optimism;");

    let mut network_choice: String = String::new();
    io::stdin().read_line(&mut network_choice)?;

    // Convert user input to network index calling the function get_network_index
    let chain_usdc = get_network_index(&network_choice).unwrap();
    let (chain, usdc_contract) = match chain_usdc {
        Network::Ethereum(chain_id, address) => (chain_id, address),
        Network::Polygon(chain_id, address) => (chain_id, address),
        Network::Optimism(chain_id, address) => (chain_id, address),
    };

    let url_chain: &str;

    if chain == 1 {
        url_chain = "mainnet"
    } else if chain == 137 {
        url_chain = "polygon-mainnet"
    } else if chain == 10 {
        url_chain = "optimism-mainnet"
    } else {
        url_chain = "err"
    }

    //Check if .env file is ok - get API_KEY
    dotenvy::dotenv().ok();
    let INFURA_API_KEY: String = dotenvy::var("INFURA_API_KEY").expect("API KEY is need");

    // Instanciate the provider given a blockchain connection
    let provider: Provider<Http> = Provider::try_from(format!(
        "https://{}.infura.io/v3/{}",
        url_chain, INFURA_API_KEY
    ))
    .map_err(|err| eyre::eyre!("Failed to create provider: {}", err))?;

    // Waiting for user input an address
    let mut address_input: String = String::new();
    println!("Digite o endereço da carteira: ");
    io::stdin()
        .read_line(&mut address_input)
        .map_err(|e| eyre::eyre!("Failed to read input: {}", e))?;

    // Parse the user input and verify if is a valid EVM address
    let address_from: ethers::types::H160 = address_input
        .trim()
        .parse::<Address>()
        .map_err(|err| eyre::eyre!("Failed to read input: {}", err))?;

    // call the print_balances - it will print the amount of USDC token
    print_balances(&provider, address_from, usdc_contract).await?;

    Ok(())
}

fn get_network_index(network_choice: &str) -> Result<Network, eyre::Error> {
    match network_choice.trim() {
        "a" => Ok(Network::Ethereum(
            1,
            "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse::<Address>()?,
        )),
        "b" => Ok(Network::Polygon(
            137,
            "0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359".parse::<Address>()?,
        )),
        "c" => Ok(Network::Optimism(
            10,
            "0x0b2c639c533813f4aa9d7837caf62653d097ff85".parse::<Address>()?,
        )),
        _ => {
            println!("Invalid network selection. Please try again.");
            Err(eyre::eyre!("Invalid network selection"))
        }
    }
}

async fn print_balances(
    provider: &Provider<Http>,
    address_from: Address,
    contract_usdc: Address,
) -> eyre::Result<()> {
    // Instanciate the ABI with an ERC20 interface
    abigen!(
        IERC20,
        r#"[
            function balanceOf(address account) external view returns (uint256)
            function decimals() public view returns (uint8)
        ]"#,
    );

    // Contract client
    let client = Arc::new(provider);
    // ERC20 contract instanciate
    let contract = IERC20::new(contract_usdc, client);

    let result: U256 = (contract.balance_of(address_from).call())
        .await
        .map_err(|e| eyre::eyre!("Failed to get balance: {}", e))?;

    if result.is_zero() {
        println!("The wallet has 0 USDC.");
        return Ok(());
    }

    let decimal: u8 = (contract.decimals().call())
        .await
        .map_err(|e| eyre::eyre!("Failed to get decimals: {}", e))?;

    let formatted_result = ethers::core::utils::format_units(result, decimal as u32)
        .map_err(|e| eyre::eyre!("Failed to format units: {}", e))?;

    let output = format!(
        "| Your wallet: {:#?} \n| has been: \n|          = {} USDC ",
        address_from, formatted_result
    );
    println!("{}", output);

    Ok(())
}
