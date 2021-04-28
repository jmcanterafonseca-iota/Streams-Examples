use iota_streams::{
  app::transport::tangle::PAYLOAD_BYTES,
  app::transport::{
    tangle::client::{iota_client, Client, SendOptions},
    TransportOptions,
  },
  app_channels::api::tangle::MessageContent::SignedPacket,
  app_channels::api::tangle::{Address, Subscriber},
  core::{
    prelude::{Rc, String}
  },
};

use anyhow::{ Result, ensure, bail };


use std::env;

use core::cell::RefCell;

use futures::executor::block_on;


fn main_function() {
  let encoding = "utf-8";

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

  // Get the arguments that were passed to the subscriber
  let args: Vec<String> = env::args().collect();

  let channel_address = &args[1];
  let announce_message_identifier = &args[2];
  let signed_message_identifier = &args[3];

  let mut subscriberA = Subscriber::new("SUBSCRIBERA9SEED", encoding, PAYLOAD_BYTES, transport.clone());

  // Link of the Announce Message that will allow the Subscriber to know how to receive messages
  let announcement_link = Address::from_str(&channel_address, &announce_message_identifier).unwrap();

  subscriberA.receive_announcement(&announcement_link).unwrap();

  println!("  SubscriberA Public Key: {:?}", subscriberA.get_pk());
  println!("Channel Address: {}", subscriberA.channel_address().unwrap());

  // Now we get a signed packet from the channel
  let message_link = Address::from_str(&channel_address, &signed_message_identifier).unwrap();
  // The public key is the public key of the Author who published the packet there
  match subscriberA.receive_signed_packet(&message_link) {
    Ok((pk, public_payload, masked_payload)) => {
      println!("Public Key Received: {:?}", pk);
      println!("Public {}", std::str::from_utf8(&public_payload.0).unwrap());
      println!("Masked {}", std::str::from_utf8(&masked_payload.0).unwrap());
    },
    Err(why) => println!("Error: {:?}", why),
  }
}


#[tokio::main]
async fn main() {
  main_function();
}
