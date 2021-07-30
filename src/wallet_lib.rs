use secp256k1::{PublicKey, SecretKey};
use anyhow::{Result};
use secp256k1::rand::{rngs, SeedableRng};
use web3::Web3;
use web3::transports::Http;
use web3::types::{TransactionParameters, U256, H256, H160};

pub fn create_keypair() -> Result<(SecretKey, PublicKey)>{
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(6);
     Ok(secp.generate_keypair(&mut rng))
}

pub fn establish_web3_connection(url: &str) -> Result<Web3<Http>>{
    let transport = web3::transports::Http::new(url)?;
    Ok(web3::Web3::new(transport))
}

pub fn create_txn_object(to: H160, value: usize)-> Result<TransactionParameters>{
  Ok(TransactionParameters {
        to: Some(to),
        //todo: check value
        value: U256::exp10(value), //0.1 eth
        ..Default::default()
    })
}

pub async fn sign_and_send(web3: Web3<Http>, tx_object: TransactionParameters, seckey: SecretKey) -> Result<H256>{
   let signed = web3.accounts().sign_transaction(tx_object, &seckey).await?;
     Ok(web3.eth().send_raw_transaction(signed.raw_transaction).await?)
}
