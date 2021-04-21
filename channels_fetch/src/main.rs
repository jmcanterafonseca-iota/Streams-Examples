use iota_streams::{
  app::transport::tangle::PAYLOAD_BYTES,
  app::transport::{
    tangle::client::{iota_client, Client, SendOptions},
    TransportOptions,
  },
  app_channels::api::tangle::{Address, Author},
  core::{
    prelude::{Rc, String},
    Result,
  },
};

use core::cell::RefCell;

use std::env;

use futures::executor::block_on;

fn main_function() {
  println!(".... Starting IOTA Streams / Channels Fetch ....");

  let encoding = "utf-8";

  // Get the channel's seed
  let args: Vec<String> = env::args().collect();

  let mut seed: &str = &String::default();
  let mut start_msg: &str = &String::default();

  if args.len() > 2 {
    seed = &args[1];
    start_msg = &args[2];
  }
  else {
    panic!("Please provide a seed an announce message id to the channel");
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

  // false indicates that no multi-branch will be used
  let mut author = Author::new(seed, encoding, PAYLOAD_BYTES, false, transport.clone());

  // The address of the channel
  let channel_address = author.channel_address().unwrap().to_string();
  println!("Channel Address: {}", channel_address);

  let messages = author.fetch_next_msgs();
  println!("Number of messages: {}", messages.len());

  for message in messages.iter() {
    println!("Message: {}", message.link);
  }

  match author.receive_msg(&Address::from_str(&channel_address, start_msg).unwrap()) {
    Ok(message) => println!("Received message: {}", message.link), 
   Err(why) => println!("Error: {:?}", why)
  }
}

#[tokio::main]
async fn main() {
  main_function();
}
