use iota_streams::app_channels::api::tangle::Author;

mod api_author;
use crate::api_author::announce::start_a_new_channel;
use crate::api_author::send_message::send_signed_message;

use iota_streams::app::transport::tangle::{
  client::{RecvOptions, SendTrytesOptions},
  PAYLOAD_BYTES,
};

use iota::client as iota_client;

use chrono::prelude::*;

use serde_json::json;

fn main() {
  println!(".... Starting IOTA Streams / Channels Application ....");

  let mut client = iota_client::Client::get();
  iota_client::Client::add_node("https://nodes.devnet.iota.org:443").unwrap();

  let mut send_opt = SendTrytesOptions::default();
  send_opt.min_weight_magnitude = 9;
  send_opt.local_pow = false;

  let encoding = "utf-8";

  let mut author = Author::new("64312STREAMSANDCHANNELS", encoding, PAYLOAD_BYTES, false);

  let channel_address = author.channel_address().unwrap().to_string();
  let announce_message = start_a_new_channel(&mut author, &mut client, send_opt).unwrap();

  println!("Channel Address: {}", channel_address);
  println!("Announce Message: {}", announce_message);

  let message = json!({
      "message": "Hello",
      "timestamp": Utc::now().to_rfc3339()
  });

  // let public_payload = r#"{ "message": "Hello World" }"#;

  let signed_message = send_signed_message(
    &mut author,
    &channel_address,
    &announce_message.msgid.to_string(),
    &mut message.to_string(),
    &mut client,
    send_opt,
  )
  .unwrap();

  println!(
    "Channel Id:  {} Announce Message Id: {} Signed Message Id: {}",
    channel_address, announce_message.msgid, signed_message.msgid
  );
}
