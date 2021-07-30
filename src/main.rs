  use web3;
  use tokio;
  use web3::types::{Address};
  use std::str::FromStr;
  use crate::wallet_lib::{create_txn_object, sign_and_send};
  use anyhow::{Result};


 mod wallet_lib;


  const URL: &str = "https://eth-ropsten.alchemyapi.io/v2/owrGyNISGOXRtlnncV2XGYZ4a6DvdYi_";

  //todo: handle basic RPC errors,
  // make it into a library with functions I can call
  // call those functions from the interface on button clicks
  // make an interface
  #[tokio::main]
   async fn main() -> Result<()> {

    let (seckey, pubkey) = wallet_lib::create_keypair()?;
    println!("public key: {}", pubkey);

    let web3 = wallet_lib::establish_web3_connection(URL)?;

    let to = Address::from_str("0x08302CF8648A961c607e3e7Bd7B7Ec3230c2A6c5").unwrap();
    let tx_object = create_txn_object(to, 7)?;

    let result =  sign_and_send(web3,tx_object,seckey).await?;

    println!("Tx succeeded with hash: {}", result);

       Ok(())
    }
