## README for the Rust Ethereum USDC Wallet CLI

### Introduction

This is a Rust-based command-line tool that allows you to check the balance of your USDC tokens in an Ethereum wallet. It connects to an Ethereum node and retrieves the balance using the `web3` library.

### Installation

1. Install Rust and Cargo: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Clone this repository: `git clone https://<your-repo-url>`
3. Install the dependencies: `cargo install`

### Usage

```bash
wallet-usd <wallet_address>
```

Replace `<wallet_address>` with the Ethereum address of the wallet you want to check the balance of.

### Example

```bash
wallet-usd 0x50EC05ADe8280758E2077fcBC08D878D4aef79C3
```

This will print the following output:

```
| Your wallet: 0x50EC05ADe8280758E2077fcBC08D878D4aef79C3 |
| Has:                                                    |
|---------------------------------------- 300.897023 USDC |
```

### Features

* Checks the balance of USDC tokens in an Ethereum wallet
* Connects to an Ethereum node using the `web3` library
* Displays the balance in a user-friendly format

### Contributing

We welcome contributions to this project! Please feel free to open issues or pull requests with suggestions or improvements.

### References

* [web3](https://docs.rs/web3/)
* [Ethereum](https://ethereum.org/)
* [Rust](https://www.rust-lang.org/)
