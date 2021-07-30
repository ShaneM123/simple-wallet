  use web3;
  use web3::signing::{Key};
  use tokio;
  use secp256k1::rand::{rngs, SeedableRng};
  use secp256k1::{PublicKey, Secp256k1, SecretKey, key};
  use web3::types::{Address, TransactionParameters, U256};
  use std::str::FromStr;
  use tiny_keccak::{Hasher, Keccak};
  const URL: &str = "https://eth-ropsten.alchemyapi.io/v2/owrGyNISGOXRtlnncV2XGYZ4a6DvdYi_";

  //todo: handle basic RPC errors,
  // make an interface
  #[tokio::main]
    async fn main() -> web3::Result<()> {

    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(6);
    let (seckey, pubkey) = secp.generate_keypair(&mut rng);
    let transport = web3::transports::Http::new(URL)?;
    let web3 = web3::Web3::new(transport);
    let to = Address::from_str("0x08302CF8648A961c607e3e7Bd7B7Ec3230c2A6c5").unwrap();
    let tx_object = TransactionParameters {
      to: Some(to),
      value: U256::exp10(17), //0.1 eth
      ..Default::default()
    };

    // Sign the tx (can be done offline)
    let signed = web3.accounts().sign_transaction(tx_object, &seckey).await?;

    // Send the tx to infura
    let result = web3.eth().send_raw_transaction(signed.raw_transaction).await?;

    println!("Tx succeeded with hash: {}", result);

        Ok(())
    }
