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

// todo
/*  > Handle errors;
   Ok Handle 0 coins in wallet;
   Ok Handle wrong address pattern - if user input a contract address return - "This is not a valid wallet address"
   > Multichain;
   - User can be able to change the chain(chainId) for those: Ethereum, Polygon, Optimism
   > UX CLI:
   - pretty formatting;
   > Using up-to-date libs;
   - This is needed because ethers.rs has been deprecated

*/

#[tokio::main]
#[allow(non_snake_case)]
async fn main() -> eyre::Result<()> {
    //Verifica se o arquivo .env está ok - captura a chave Infura
    dotenvy::dotenv().ok();
    let INFURA_API_KEY = dotenvy::var("INFURA_API_KEY").expect("API KEY is need");

    // Instancia o provider dada uma conexão com a Ethereum mainnet
    let provider = Provider::try_from(format!("https://mainnet.infura.io/v3/{}", INFURA_API_KEY))
        .map_err(|err| eyre::eyre!("Failed to create provider: {}", err))?;

    // Aguarda o input do usuário
    let mut address_input = String::new();
    println!("Digite o endereço da carteira: ");
    io::stdin()
        .read_line(&mut address_input)
        .map_err(|e| eyre::eyre!("Failed to read input: {}", e))?;

    // "Trata" o input do usuário verificando se é ou não um address de uma wallet
    let address_from = address_input
        .trim()
        .parse::<Address>()
        .map_err(|err| eyre::eyre!("Failed to read input: {}", err))?;

    // chama a função para imprimir o saldo da carteira, dado um provider e o endereço
    print_balances(&provider, address_from).await?;

    Ok(())
}

async fn print_balances(provider: &Provider<Http>, address_from: Address) -> eyre::Result<()> {
    // Endereço do contrato USDC no Ethereum Mainnet
    let token_address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse::<Address>()?;

    // Instância do ABI com Interface de um ERC20
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

    // Cliente do contrato
    let client = Arc::new(provider);
    // Crie instância do contrato ERC20
    let contract = IERC20::new(token_address, client);

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
