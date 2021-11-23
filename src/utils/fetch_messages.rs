use iota_streams::{
    app_channels::api::tangle::{
        Author,
        Subscriber,
        Transport,
    },
};
use iota_streams::app_channels::api::tangle::MessageContent;

pub async fn s_fetch_next_messages<T: Transport>(subscriber: &mut Subscriber<T>) {
    let msgs = subscriber.fetch_all_next_msgs().await;
    let processed_msgs = msgs
    .iter()
    .map(|msg| {
        let content = &msg.body;
        match content {
            MessageContent::SignedPacket {
                pk: _,
                public_payload: _,
                masked_payload,
            } => String::from_utf8(masked_payload.0.to_vec()).unwrap(),
            _ => String::default(),
        }
    })
    .filter(|s| s != &String::default())
    .collect::<Vec<String>>();


    print!("Retrieved messages: ");
    for i in 0..processed_msgs.len() {
        print!("{}, ", processed_msgs[i]);
    }
    println!();
}

pub async fn _a_fetch_next_messages<T: Transport>(author: &mut Author<T>) {
    let msgs = author.fetch_all_next_msgs().await;
    let processed_msgs = msgs
    .iter()
    .map(|msg| {
        let content = &msg.body;
        match content {
            MessageContent::SignedPacket {
                pk: _,
                public_payload: _,
                masked_payload,
            } => String::from_utf8(masked_payload.0.to_vec()).unwrap(),
            _ => String::default(),
        }
    })
    .filter(|s| s != &String::default())
    .collect::<Vec<String>>();


    print!("Retrieved messages: ");
    for i in 0..processed_msgs.len() {
        print!("{}, ", processed_msgs[i]);
    }
    println!();
}
