  use secp256k1::{PublicKey, SecretKey};
use web3;
  use tokio;
  use web3::types::{Address};
  use core::panic;
use std::{collections::HashMap, str::FromStr};
  use crate::wallet_lib::{create_txn_object, sign_and_send};
  use anyhow::{Result};
  use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};


  mod wallet_lib;


  const URL: &str = "https://eth-ropsten.alchemyapi.io/v2/owrGyNISGOXRtlnncV2XGYZ4a6DvdYi_";

  //todo: handle basic RPC errors,
  // make it into a library with functions I can call
  // call those functions from the interface on button clicks
  // make an interface
#[derive(Clone,Debug)]
  pub enum WalletMessage{
    NewWallet,
  }

  #[tokio::main]
   async fn main() -> Result<()> {
      let app = app::App::default();
      let mut wind = Window::default().with_size( 1200, 800).with_label("Simple Wallet");
      let mut but = Button::new(100, 100, 200, 300, "Create Wallet");
      let mut frame = Frame::default()
      .with_size(600, 100)
      .center_of(&wind)
      .with_label("0 Wallets");
      let web3 = wallet_lib::establish_web3_connection(URL)?;
      let mut accounts = web3.eth().accounts().await?;
      let mut keypairs: Vec<(PublicKey, SecretKey)> = Vec::new();
      wind.end();
      wind.show();

    let (s, r) = app::channel::<WalletMessage>();

    but.emit(s, WalletMessage::NewWallet);
    // but_dec.emit(s, Message::Decrement);
    
    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                WalletMessage::NewWallet => {
                        let (seckey, pubkey) =
      match  wallet_lib::create_keypair() {
        Ok(val) => val,
        //todo: implement anyhow error handling here
        Err(e) => unimplemented!(),
      };
                  keypairs.push((pubkey, seckey));
                  frame.set_label(&format!("{} Wallets", keypairs.len()));
                  println!("keypairs {:?}", keypairs);
                },
            }
        }
    }
     app.run().unwrap();
    // let (seckey, pubkey) = wallet_lib::create_keypair()?;
    // println!("public key: {}", pubkey);
    //
    //
    // let to = Address::from_str("0x08302CF8648A961c607e3e7Bd7B7Ec3230c2A6c5").unwrap();
    // let tx_object = create_txn_object(to, 7)?;
    //
    // let result =  sign_and_send(web3,tx_object,seckey).await?;

    //println!("Tx succeeded with hash: {}", result);

       Ok(())
    }




  // pub struct FlatButton {
  //     frm: frame::Frame,
  // }
  //
  // impl FlatButton {
  //     pub fn new(w: i32, h: i32, title: &str) {
  //         let w: FlatButton = FlatButton {
  //             frm: frame::Frame::new(0,0,w,h,titel)
  //         };
  //         //todo: move to another function
  //         w.frm.set_frame(FrameType::RFlatBox);
  //         w.frm.set_color(Color::Red);
  //         let mut w_c = w.clone();
  //         w.frm.handle(Box::new(move |ev| match ev{
  //             Event::Push => {
  //                 if w.color() => Color::Green {
  //                     w_c.set_color(Color::Red);
  //                 }
  //                 else {
  //                     w_c.set_color(Color::Green)
  //                 }
  //             },
  //             _ => false,
  //         }))
  //
  //     }
  // }
