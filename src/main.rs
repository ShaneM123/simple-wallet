  use web3;
  use web3::signing::{Key};
  use tokio;
  use secp256k1::rand::rngs::OsRng;
  use secp256k1::{PublicKey, Secp256k1, SecretKey};
  use web3::types::{Address, TransactionParameters, U256};
  use std::str::FromStr;
  use tiny_keccak::{Hasher, Keccak};

  const URL: &str = "https://eth-ropsten.alchemyapi.io/v2/owrGyNISGOXRtlnncV2XGYZ4a6DvdYi_";

  #[tokio::main]
    async fn main() -> web3::Result<()> {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = OsRng::new().unwrap();
    let (seckey, pubkey) = secp.generate_keypair(&mut rng);


    // Sign up at infura > choose the desired network (eg Rinkeby) > copy the endpoint url into the below
    // If you need test ether use a faucet, eg https://faucet.rinkeby.io/
    let transport = web3::transports::Http::new(URL)?;
    let web3 = web3::Web3::new(transport);

    // Insert the 20-byte "to" address in hex format (prefix with 0x)
    let to = Address::from_str("0xXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX").unwrap();

    // Insert the 32-byte private key in hex format (do NOT prefix with 0x)
    let prvk = SecretKey::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX").unwrap();

    // Build the tx object
    let tx_object = TransactionParameters {
      to: Some(to),
      value: U256::exp10(17), //0.1 eth
      ..Default::default()
    };

    // Sign the tx (can be done offline)
    let signed = web3.accounts().sign_transaction(tx_object, &prvk).await?;

    // Send the tx to infura
    let result = web3.eth().send_raw_transaction(signed.raw_transaction).await?;

    println!("Tx succeeded with hash: {}", result);

        Ok(())
    }
