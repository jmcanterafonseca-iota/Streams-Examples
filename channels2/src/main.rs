use iota::client as iota_client;

use iota_streams::app::transport::tangle::{
  client::{RecvOptions, SendTrytesOptions},
  PAYLOAD_BYTES,
};

use iota_streams::app_channels::{
  api::tangle::{Subscriber}
};

use std::env;

mod api_subscriber;
use crate::api_subscriber::subscribe::get_announcement;
use crate::api_subscriber::subscribe::get_signed_messages;


fn main() {
  let encoding = "utf-8";
  let mut subscriber = Subscriber::new("XYZ567ABC012", encoding, PAYLOAD_BYTES);

  // Connect to a node
  let mut client = iota_client::Client::get();
  iota_client::Client::add_node("https://nodes.devnet.iota.org:443").unwrap();

  // Get the arguments that were passed to the subscriber
  let args: Vec<String> = env::args().collect();

  let channel_address = &args[1];
  let announce_message_identifier = &args[2];
  let signed_message_identifier = &args[3];

  let recv_opt = RecvOptions::default();

  match get_announcement(
    &mut subscriber,
    &channel_address,
    &announce_message_identifier,
    &mut client,
    recv_opt,
  ) {
    Ok(()) => (),
    Err(error) => println!("Failed with error {}", error),
  }

  match get_signed_messages(
    &mut subscriber,
    &channel_address,
    &signed_message_identifier,
    &mut client,
    recv_opt,
  ) {
    Ok(()) => (),
    Err(error) => println!("Failed with error {}", error),
  }
}
