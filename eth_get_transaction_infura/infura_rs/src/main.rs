use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    prelude::*
};
use eyre::Result;
use std::convert::TryFrom;
use dotenv::dotenv;
use std::collections::HashMap;

//adding tests
#[cfg(test)]
mod tests;

// This clode does eth_look of the Guardian components
// Using the infura service for Ethereum API calls https://www.infura.io/
// This code should support ethereum sepolia, holesky and mainnet networks
// This code should be able to get the transaction details from the network via the transaction hash
// Interface should be async fn lookup(ethereum_chain_id, tx_hash) -> (timestamp, event_hash)

//Parse the received data into serde_json
//Provide an internal interface

// Test-data inputs to verify functionality of the function
// (mainnet, 0x9d4897d3e381982ee872cb193469d991cce8d087f0cd5fe275926f80c1326a1e)
// (holesky, 0xe20ee33fe150423099d6c22bf84683e19d03e40371e2c76e59293d026e8d0101)
// (sepolia, 0xae9b476d8eed73897b0f71ac59c267856dbae64f249518fea862377208436cc5)

fn chain_id_to_url() -> HashMap<u32, &'static str> {
    let mut map = HashMap::new();
    map.insert(0x1, "https://mainnet.infura.io/v3/");
    map.insert(0x4268, "https://holesky.infura.io/v3/");
    map.insert(0xaa36a7, "https://sepolia.infura.io/v3/");
    map
}

async fn get_tx_data(chain_id:u32, tx_hash:&str) -> Result<()> {

    let my_map = chain_id_to_url(); //retrieve the HashMap

    if let Some(url_prefix) = my_map.get(&chain_id) {
        println!("The network name for chain ID is {}", url_prefix);
    }

    //load .env file
    dotenv().ok();

    //Load infura API key from .env file
    let infura_api_key = std::env::var("INFURA_API_KEY").expect("INFURA_API_KEY must be set");
    //build URL
    let url_prefix = my_map.get(&chain_id).unwrap();
    //println!("The network name for chain ID 0x1 is {}", url_prefix);
    

    let url = url_prefix.to_string() + &infura_api_key.to_string();
    println!("The URL is {}", url);
    // Connect to the network via Infura
    
    //let url = format!("https://mainnet.infura.io/v3/{}", infura_api_key);
    //let provider = Provider::<Http>::try_from(url.as_str())?;

    let provider = Provider::<Http>::try_from(url)?;
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
   //let transaction_hash: H256 ="0x9d4897d3e381982ee872cb193469d991cce8d087f0cd5fe275926f80c1326a1e".parse().unwrap();
   let transaction_hash: H256 =tx_hash.parse().unwrap();

   let tx = client.get_transaction(transaction_hash).await?;

   println!("Sent tx: {}\n", serde_json::to_string(&tx)?);

   Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
//mainnet
//Ok(get_tx_data(0x1,"0x9d4897d3e381982ee872cb193469d991cce8d087f0cd5fe275926f80c1326a1e").await?)
//holesky
//Ok(get_tx_data(0x4268,"0xe20ee33fe150423099d6c22bf84683e19d03e40371e2c76e59293d026e8d0101").await?)
//sepolia
Ok(get_tx_data(0xaa36a7,"0xae9b476d8eed73897b0f71ac59c267856dbae64f249518fea862377208436cc5").await?)
}