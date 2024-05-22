use web3::eth::Eth;
use web3::types::{Address, U64};

fn main() {
    // Conecte-se ao nó Ethereum
    let web3 = web3::Web3::new("http://localhost:8545");
    let eth = web3.eth();

    // Obtenha o endereço da carteira do cliente
    let carteira_str = std::env::args()
        .nth(1)
        .expect("Endereço da carteira não fornecido");
    let carteira = Address::from_str(&carteira_str).expect("Endereço da carteira inválido");

    // Obtenha o saldo de USDC
    let saldo = eth
        .get_balance(carteira, None)
        .expect("Falha ao obter saldo");
    let saldo_usd = saldo.as_u64() / 1e18;

    // Exiba o resultado
    println!("| Sua carteira: {} |", carteira_str);
    println!("| Possui: |");
    println!(
        "|---------------------------------------------- {:.6} USDC",
        saldo_usd
    );

    // connect to the network, don't forget to replace your INFURA_API_KEY
    let provider = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/3740d118c75f4da08308dea88e9d932a",
    )?;

    let chain_id = provider.get_chainid().await?;

    // define a `ERC20Contract` struct from the ABI
    abigen!(ERC20Contract, "./contract_abi.json");

    let token_contract = "0x6b175474e89094c44da98b954eedeac495271d0f".parse::<Address>()?;
    let token_holder = "0xfa4b329b7892e5d7663d4c1c005a5cd5f403f564".parse::<Address>()?;

    // let contract = ERC20Contract::new(contract_address, signer);
    let contract = ERC20Contract::new(token_contract, Arc::new(token_holder));

    // get the desired amount of tokens to the `to_address`
    let tx = contract.balance_of(token_holder);
    let pending_tx = tx.send().await?;
    let _mined_tx = pending_tx.await?;

    println!("balance dai token: {}", serde_json::to_string(&_mined_tx)?);
}
