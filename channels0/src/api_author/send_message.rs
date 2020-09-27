use anyhow::{bail, Result};
use iota_streams::app_channels::api::tangle::{Address, Author, Transport};
use iota_streams::ddml::types::Bytes;

pub fn send_signed_message<T: Transport>(
  author: &mut Author,
  channel_address: &String,
  announce_message_identifier: &String,
  public_payload: &String,
  client: &mut T,
  send_opt: T::SendOptions,
) -> Result<Address>
where
  T::SendOptions: Copy,
{
  let public_payload = Bytes(public_payload.as_bytes().to_vec());

  let announcement_link = match Address::from_str(&channel_address, &announce_message_identifier) {
    Ok(announcement_link) => announcement_link,
    Err(()) => bail!(
      "Failed to create Address from {}:{}",
      &channel_address,
      &announce_message_identifier
    ),
  };

  let empty_masked_payload = Bytes("".as_bytes().to_vec());
  let message = author.sign_packet(&announcement_link, &public_payload, &empty_masked_payload)?;

  client.send_message_with_options(&message.0, send_opt)?;

  println!("Published signed message ....");

  Ok(message.0.link)
}
