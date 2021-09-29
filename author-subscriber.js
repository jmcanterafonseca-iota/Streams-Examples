const { Author, Subscriber, SendOptions, ChannelType, streamsPanicHook } = require("@tangle.js/streams-wasm/node");
const fetch = require('node-fetch');

function initialize() {
    // @ts-expect-error Streams WASM bindings need it
    global.fetch = fetch;
    // @ts-expect-error  Streams WASM bindings need it
    global.Headers = fetch.Headers;
    // @ts-expect-error  Streams WASM bindings need it
    global.Request = fetch.Request;
    // @ts-expect-error  Streams WASM bindings need it
    global.Response = fetch.Response;

    // streamsPanicHook();
}

async function main() {
    const options = new SendOptions("https://api.lb-0.h.chrysalis-devnet.iota.cafe", true);
    const seed = make_seed(20);
    const auth = new Author(seed,options.clone(),ChannelType.SingleBranch);

    let response = await auth.clone().send_announce();
    let ann_link = response.link;
    const ann_link_copy = ann_link.copy();
    console.log("announced at: ", ann_link.toString());
    console.log("Announce message index: " + ann_link.toMsgIndexHex());
  
    let sub = new Subscriber(seed, options.clone());
    const announce = await sub.clone().receive_announcement(ann_link_copy.copy());

    const response2 = await sub.clone().send_subscribe(ann_link_copy.copy());
    await auth.clone().receive_subscribe(response2.link.copy());

    const result = await auth.clone().send_keyload_for_everyone(ann_link_copy.copy());
    const key_load_link_copy = result.link.copy();
    
    const keyLoad = await sub.clone().receive_keyload(key_load_link_copy.copy());
    console.log("Keyload received: ", keyLoad);

    const response_msg = await sub.clone().send_signed_packet(
        key_load_link_copy.copy(),Buffer.from(""),Buffer.from("hello"));

    let seed3 = seed;
    let sub3 = new Subscriber(seed3, options.clone());
    const announce3 = await sub3.clone().receive_announcement(ann_link_copy.copy());
    const keyLoad3 = await sub3.clone().receive_keyload(key_load_link_copy.copy());
    console.log("Keyload received by Subscriber 3: ", keyLoad3);

    const messages_3 = await sub3.clone().fetch_next_msgs();
    console.log("Fetch next messages subscriber 3:", messages_3.length);

    const subs_msg3 = await sub3.clone().receive_msg(response_msg.link.copy());
    console.log("Message received by Subscriber 3: ", Buffer.from(subs_msg3.message.get_masked_payload()).toString());
}

function make_seed(size) {
    const alphabet = "abcdefghijklmnopqrstuvwxyz";
    let seed = "";
    for (i = 9; i < size; i++) {
      seed += alphabet[Math.floor(Math.random() * alphabet.length)];
    }
    return seed;
}

initialize();
main().then(() => console.log("Done")).catch(e => console.error(e));
