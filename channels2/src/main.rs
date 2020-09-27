use iota::client as iota_client;

use iota_streams::app::transport::tangle::{
  client::{RecvOptions, SendTrytesOptions},
  PAYLOAD_BYTES,
};
use iota_streams::app_channels::{
  api::tangle::{Address, Subscriber, Transport},
  message,
};

use anyhow::{bail, ensure, Result};
use std::env;

fn find_message_link_opt_sequence<T: Transport>(
  subscriber: &mut Subscriber,
  address: &String,
  message_identifier: &String,
  client: &mut T,
  recv_opt: T::RecvOptions,
) -> Result<Address>
where
  T::RecvOptions: Copy,
{
  let message_link = match Address::from_str(&address, &message_identifier) {
    Ok(message_link) => message_link,
    Err(()) => bail!(
      "Failed to create Address from {}:{}",
      &address,
      &message_identifier
    ),
  };

  Ok(message_link)
}

fn get_announcement<T: Transport>(
  subscriber: &mut Subscriber,
  channel_address: &String,
  announce_message_identifier: &String,
  client: &mut T,
  recv_opt: T::RecvOptions,
) -> Result<()>
where
  T::RecvOptions: Copy,
{
  let announcement_link = find_message_link_opt_sequence(
    subscriber,
    channel_address,
    announce_message_identifier,
    client,
    recv_opt,
  )?;
  let list = client.recv_messages_with_options(&announcement_link, recv_opt)?;

  for tx in list.iter() {
    let header = tx.parse_header()?;
    ensure!(
      header.check_content_type(message::ANNOUNCE),
      "Content type should be announce type"
    );

    subscriber.unwrap_announcement(header.clone())?;

    println!(
      "Found and verified announcement message of type {}",
      header.content_type()
    );
  }

  Ok(())
}

fn get_signed_messages<T: Transport>(
  subscriber: &mut Subscriber,
  channel_address: &String,
  signed_message_identifier: &String,
  client: &mut T,
  recv_opt: T::RecvOptions,
) -> Result<()>
where
  T::RecvOptions: Copy,
{
  let message_link = find_message_link_opt_sequence(
    subscriber,
    channel_address,
    signed_message_identifier,
    client,
    recv_opt,
  )?;

  let list = client.recv_messages_with_options(&message_link, recv_opt)?;

  for tx in list.iter() {
    let header = tx.parse_header()?;
    ensure!(
      header.check_content_type(message::SIGNED_PACKET),
      "Content type should be signed type"
    );

    let (public_payload, masked_payload) = match subscriber.unwrap_signed_packet(header.clone()) {
      Ok(result) => (result.1, result.2),
      Err(_) => bail!("Error unwrapping packet"),
    };

    let pub_msg = String::from_utf8(public_payload.0).unwrap();
    let priv_msg = String::from_utf8(masked_payload.0).unwrap();

    println!("Public message: {}", pub_msg);
    println!("Private message: {}", priv_msg);
  }
  
  Ok(())
}

fn main() {
  let encoding = "utf-8";
  let mut subscriber = Subscriber::new("XYZ567ABC012", encoding, PAYLOAD_BYTES);

  // Connect to a node
  let mut client = iota_client::Client::get();
  iota_client::Client::add_node("https://nodes.devnet.iota.org:443").unwrap();

  // Get the arugments that were passed to the subscriber
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
