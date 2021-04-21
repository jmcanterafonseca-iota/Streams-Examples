mod api_author;

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
  ddml::types::Bytes,
};

use core::cell::RefCell;

use chrono::prelude::Utc;

use serde_json::json;

use rand::{
  distributions::Alphanumeric,
  {thread_rng, Rng},
};

use std::env;

use futures::executor::block_on;

fn main_function() {
  println!(".... Starting IOTA Streams / Channels Application ....");

  let encoding = "utf-8";

  // Get the channel's seed
  let args: Vec<String> = env::args().collect();
  let mut is_new_channel = true;

  let mut seed: &str = &generate_seed().unwrap();
  let mut previous_message_id: String = String::from("");
  let mut previous_msg_link: Address = Address::default();

  if args.len() > 1 {
    seed = &args[1];
    previous_message_id = args[2].to_owned();
    is_new_channel = false;
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

  if is_new_channel {
    previous_msg_link = author.send_announce().unwrap();
    previous_message_id = previous_msg_link.msgid.to_string();
    println!("Announce Message Id: {}", previous_message_id);
  } else {
    previous_msg_link = Address::from_str(&channel_address, &previous_message_id).unwrap();
    println!("Previous Message Id: {}", previous_message_id);
  }

  let message = json!({
      "message": "Hello",
      "timestamp": Utc::now().to_rfc3339()
  });

  // let public_payload = r#"{ "message": "Hello World" }"#;
  let public_payload = Bytes(message.to_string().as_bytes().to_vec());
  let empty_masked_payload = Bytes("".as_bytes().to_vec());

  println!("Sending message ... {}", previous_msg_link);

  match author.send_signed_packet(&previous_msg_link, &public_payload, &empty_masked_payload) {
    Err(why) => println!("Error: {:?}", why),
    Ok((signed_message, seq)) => println!(
      "Channel Id:  {} Previous Message Id: {} Signed Message Id: {}",
      channel_address, previous_message_id, signed_message.msgid
    ),
  }
}

fn generate_seed() -> Result<String> {
  let rand_string: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .map(char::from)
    .collect();

  Ok(rand_string)
}

#[tokio::main]
async fn main() {
  // main_pure();
  main_function();
}
