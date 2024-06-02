use ethers::prelude::abigen;
use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};
use eyre::Result;
use std::sync::Arc;

abigen!(
    IERC20,
    r#"[
        function balanceOf(address account) external view returns (uint256)
        function decimals() public view returns (uint8)
    ]"#,
);

pub async fn get_balance(
    provider: &Provider<Http>,
    address: Address,
    contract: Address,
) -> Result<String> {
    let client = Arc::new(provider.clone());
    let erc20 = IERC20::new(contract, client);

    let balance: U256 = erc20.balance_of(address).call().await?;
    let decimals: u8 = erc20.decimals().call().await?;

    if balance.is_zero() {
        return Ok("The wallet has 0 USDC.".to_string());
    }

    let formatted_balance = ethers::core::utils::format_units(balance, decimals as u32)?;
    Ok(format!(
        "Address: {:#?} has {} USDC",
        address, formatted_balance
    ))
}
