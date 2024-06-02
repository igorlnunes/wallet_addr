use crate::app::{App, AppState, Network};
use crate::network::get_balance;
use crossterm::event::{self, Event, KeyCode};
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use eyre::Result;
use std::convert::TryFrom;
use std::io::{Error, ErrorKind};
use std::time::{Duration, Instant};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

#[allow(non_snake_case)]
pub async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App<'_>) -> Result<()> {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    let messages = vec![
        "Insira sua carteira Ethereum...",
        "Insira sua carteira Ethereum.",
        "Insira sua carteira Ethereum..",
        "Insira sua carteira Ethereum...",
    ];
    let mut idx = 0;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            match app.state {
                AppState::SelectNetwork => {
                    let block = Block::default()
                        .title("Escolha sua rede blockchain:")
                        .borders(Borders::ALL);

                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(4)
                        .constraints(
                            [
                                Constraint::Length(5), // Define a altura do Block como 5 linhas
                                Constraint::Min(0),
                            ]
                            .as_ref(),
                        )
                        .split(size);

                    let items: Vec<Spans> = app
                        .items
                        .iter()
                        .enumerate()
                        .map(|(i, &item)| {
                            if i == app.selected {
                                Spans::from(vec![Span::styled(
                                    format!(">> {}", item),
                                    Style::default()
                                        .fg(Color::Yellow)
                                        .add_modifier(Modifier::BOLD),
                                )])
                            } else {
                                Spans::from(vec![Span::raw(format!("- {}", item))])
                            }
                        })
                        .collect();

                    let paragraph = Paragraph::new(items)
                        .block(block)
                        .alignment(Alignment::Left);

                    f.render_widget(paragraph, chunks[0]);
                }
                AppState::EnterAddress => {
                    let block = Block::default()
                        .title("Digite o endereço da wallet:")
                        .borders(Borders::ALL);

                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Percentage(45),
                                Constraint::Percentage(10),
                                Constraint::Percentage(45),
                            ]
                            .as_ref(),
                        )
                        .split(size);

                    let input = Spans::from(vec![Span::raw(app.address_input.as_str())]);

                    let message = vec![Spans::from(Span::raw(messages[idx]))];

                    let input_paragraph =
                        Paragraph::new(input).block(Block::default().borders(Borders::NONE));
                    f.render_widget(input_paragraph, chunks[1]);

                    let paragraph = Paragraph::new(message)
                        .block(block)
                        .alignment(Alignment::Left);

                    f.render_widget(paragraph, chunks[0]);
                }
                AppState::ShowBalances => {
                    let block = Block::default()
                        .title("Saldo da Wallet")
                        .borders(Borders::ALL);

                    let paragraph = Paragraph::new(app.balance_output.as_str())
                        .block(block)
                        .alignment(Alignment::Left);

                    f.render_widget(paragraph, size);
                }
            }
        })?;

        if crossterm::event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                match app.state {
                    AppState::SelectNetwork => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Char('e') => {
                            app.selected_network = Some(match app.selected {
                                0 => Network::Ethereum(
                                    1,
                                    "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
                                        .parse::<Address>()
                                        .unwrap(),
                                ),
                                1 => Network::Polygon(
                                    137,
                                    "0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359"
                                        .parse::<Address>()
                                        .unwrap(),
                                ),
                                2 => Network::Optimism(
                                    10,
                                    "0x0b2c639c533813f4aa9d7837caf62653d097ff85"
                                        .parse::<Address>()
                                        .unwrap(),
                                ),
                                _ => unreachable!(),
                            });
                            app.state = AppState::EnterAddress;
                        }
                        _ => {}
                    },
                    AppState::EnterAddress => match key.code {
                        KeyCode::Char(c) => {
                            app.address_input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.address_input.pop();
                        }
                        KeyCode::Enter => {
                            if let Some(selected_network) = &app.selected_network {
                                let (chain, usdc_contract) = match selected_network {
                                    Network::Ethereum(chain_id, address) => (chain_id, address),
                                    Network::Polygon(chain_id, address) => (chain_id, address),
                                    Network::Optimism(chain_id, address) => (chain_id, address),
                                };

                                let url_chain: &str;

                                if *chain == 1 {
                                    url_chain = "mainnet"
                                } else if *chain == 137 {
                                    url_chain = "polygon-mainnet"
                                } else if *chain == 10 {
                                    url_chain = "optimism-mainnet"
                                } else {
                                    url_chain = "err"
                                }

                                // Check if .env file is ok - get API_KEY

                                dotenvy::dotenv().ok();
                                let INFURA_API_KEY: String =
                                    dotenvy::var("INFURA_API_KEY").expect("API KEY is needed");

                                // Instantiate the provider given a blockchain connection
                                let provider: Provider<Http> = Provider::try_from(format!(
                                    "https://{}.infura.io/v3/{}",
                                    url_chain, INFURA_API_KEY
                                ))
                                .map_err(|err| Error::new(ErrorKind::Other, err))?;

                                app.provider = Some(provider);

                                let address_from: Address = app
                                    .address_input
                                    .trim()
                                    .parse::<Address>()
                                    .map_err(|err| Error::new(ErrorKind::Other, err))?;

                                if let Some(ref provider) = app.provider {
                                    app.balance_output =
                                        get_balance(provider, address_from, *usdc_contract).await?;
                                }

                                app.state = AppState::ShowBalances;
                            }
                        }
                        KeyCode::Esc => {
                            app.state = AppState::SelectNetwork;
                            app.address_input.clear();
                        }
                        _ => {}
                    },
                    AppState::ShowBalances => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Esc => {
                            app.state = AppState::SelectNetwork;
                            app.address_input.clear();
                        }
                        _ => {}
                    },
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
            idx = (idx + 1) % messages.len();
        }
    }
}
