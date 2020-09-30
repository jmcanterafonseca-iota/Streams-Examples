use anyhow::{ Result, ensure, bail };

use iota_streams::app_channels::{
  api::tangle::{Address, Subscriber, Transport},
  message,
};

fn find_message_link_opt_sequence<T: Transport>(
  address: &String,
  message_identifier: &String,
  _client: &mut T,
  _recv_opt: T::RecvOptions,
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

pub fn get_announcement<T: Transport>(
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

pub fn get_signed_messages<T: Transport>(
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