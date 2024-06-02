use ethers::providers::{Http, Provider};
use ethers::types::Address;

#[derive(Debug)]
pub enum Network {
    Ethereum(u8, Address),
    Polygon(u8, Address),
    Optimism(u8, Address),
}

pub enum AppState {
    SelectNetwork,
    EnterAddress,
    ShowBalances,
}

pub struct App<'a> {
    pub items: Vec<&'a str>,
    pub selected: usize,
    pub address_input: String,
    pub provider: Option<Provider<Http>>,
    pub state: AppState,
    pub selected_network: Option<Network>,
    pub balance_output: String,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            items: vec!["Ethereum", "Polygon", "Optimism"],
            selected: 0,
            address_input: String::new(),
            provider: None,
            state: AppState::SelectNetwork,
            selected_network: None,
            balance_output: String::new(),
        }
    }

    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
    }

    pub fn previous(&mut self) {
        if self.selected == 0 {
            self.selected = self.items.len() - 1;
        } else {
            self.selected -= 1;
        }
    }
}
