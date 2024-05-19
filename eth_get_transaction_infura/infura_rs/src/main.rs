use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    prelude::*
};
use eyre::Result;
use std::convert::TryFrom;
use dotenv::dotenv;

// This clode does eth_look of the Guardian components
// Using the infura service for Ethereum API calls https://www.infura.io/
// This code should support ethereum sepolia, holesky and mainnet networks
// This code should be able to get the transaction details from the network via the transaction hash
// Interface should be async fn lookup(ethereum_chain_id, tx_hash) -> (timestamp, event_hash)

//Parse the received data into serde_json
//Provide an internal interface

#[tokio::main]
async fn main() -> Result<()> {

    //load .env file
    dotenv().ok();

    //Load infura API key from .env file
    let infura_api_key = std::env::var("INFURA_API_KEY").expect("INFURA_API_KEY must be set");
    //build URL
    let url = format!("https://mainnet.infura.io/v3/{}", infura_api_key);
    // Connect to the network via Infura
    let provider = Provider::<Http>::try_from(url.as_str())?;

    //This is an alternative way to connect to the network via Infura with an Public API key
    //let provider = Provider::<Http>::try_from("https://mainnet.infura.io/v3/3d110a0fce9e49b08d2ee584e19a05ba")?;

    let chain_id = provider.get_chainid().await?;

    // Define the signer.
    // Define the the SIGNER_PRIVATE_KEY with 
    // the private key of your Ethereum account (without the 0x prefix) in the .env file. 
    let wallet_key = std::env::var("SIGNER_PRIVATE_KEY").expect("SIGNER_PRIVATE_KEY must be set");
    let wallet: LocalWallet = wallet_key.parse::<LocalWallet>()?
    .with_chain_id(chain_id.as_u64());

    // connect the wallet to the provider
    let client = SignerMiddleware::new(provider, wallet);

    //Ethereum_types {H160,H256,U256} https://kauri.io/#collections/A%20Hackathon%20Survival%20Guide/sending-ethereum-transactions-with-rust/
   let transaction_hash: H256 ="0x9d4897d3e381982ee872cb193469d991cce8d087f0cd5fe275926f80c1326a1e".parse().unwrap();

   let tx = client.get_transaction(transaction_hash).await?;

   println!("Sent tx: {}\n", serde_json::to_string(&tx)?);

   Ok(())
}
