use iota_streams::{
  app::transport::tangle::PAYLOAD_BYTES,
  app::transport::{
    tangle::client::{iota_client, Client, SendOptions},
    TransportOptions,
  },
  app_channels::api::tangle::MessageContent::SignedPacket,
  app_channels::api::tangle::{Address, Author},
  core::{
    prelude::{Rc, String},
    print, println, Result,
  },
};

use core::cell::RefCell;

use std::env;

use futures::executor::block_on;

fn main_function() {
  println!(".... Starting IOTA Streams / Channels Fetch ....");

  let password = "insecurepassword";
  let author_state_file_name = "author_state.bin";

  let encoding = "utf-8";

  // Get the channel's seed
  let args: Vec<String> = env::args().collect();

  let mut seed: &str = &String::default();
  let mut start_msg: &str = &String::default();

  if args.len() > 1 {
    start_msg = &args[1];
  } else {
    panic!("Please provide a message id to be retrieved from the channel");
  }

  // Parse env vars with a fallback
  let node_url = String::from("https://api.lb-0.testnet.chrysalis2.com");

  let send_opt = SendOptions::default();

  // Fails at unwrap when the url isnt working
  // TODO: Fail gracefully
  let iota_client = block_on(
    iota_client::ClientBuilder::new()
      .with_node(&node_url)
      .unwrap()
      //.with_node_sync_disabled()
      .with_local_pow(false)
      .finish(),
  )
  .unwrap();

  let client = Client::new(send_opt, iota_client);
  let mut transport = Rc::new(RefCell::new(client));
  transport.set_send_options(send_opt);

  println!("Seed: {}", seed);

  let author_state = std::fs::read(author_state_file_name).unwrap();
  let mut author = Author::import(&author_state, password, transport).unwrap();

  // false indicates that no multi-branch will be used
  // let mut author = Author::new(seed, encoding, PAYLOAD_BYTES, false, transport.clone());

  // The address of the channel
  let channel_address = author.channel_address().unwrap().to_string();
  println!("Channel Address: {}", channel_address);

  let message = author.receive_msg(&Address::from_str(&channel_address, start_msg).unwrap()).unwrap();

  match &message.body {
    SignedPacket {
      pk,
      public_payload,
      masked_payload,
    } => {
      println!("Public Key: {:#?}", pk);
      println!("Message Id {}", message.link);
      // println!("Previous Message Id {}", message.);
      println!("Public {}", std::str::from_utf8(&public_payload.0).unwrap());
      println!("Masked {}", std::str::from_utf8(&masked_payload.0).unwrap());
    }
    _ => {}
  }
}

#[tokio::main]
async fn main() {
  main_function();
}
