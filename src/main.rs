use futures::executor::block_on;
use iota_streams::app::transport::tangle::client::iota_client::Client as OtherClient;
use iota_streams::app::transport::tangle::client::{Client, SendOptions};
use iota_streams::app_channels::api::tangle::Author;
use iota_streams::app_channels::api::tangle::ChannelType;
use iota_streams::app_channels::api::tangle::Subscriber;
use iota_streams::ddml::types::Bytes;
mod utils;
use crate::utils::ramdon_seed::random_seed;

#[tokio::main]
async fn main() {
    dotenv::from_path("./development.env").ok();

    println!("\nCreate Transport Client");
    let send_options: SendOptions = SendOptions {
        url: std::env::var("NODE").unwrap(),
        local_pow: false,
    };
    
    let iota_client = block_on(
        OtherClient::builder()
          .with_node(&std::env::var("NODE").unwrap())
          .unwrap()
          .with_local_pow(false)
          .finish(),
    ).unwrap();
    
    let client = Client::new(send_options, iota_client);
    
    println!("\nCreate Author");
    let mut author = Author::new(&random_seed(64),ChannelType::SingleBranch ,client.clone());

    println!("\nAuthor Announce Channel");
    let announcement_link = {
        let msg = author.send_announce().await.unwrap();
        println!("  msg => <{}> <{:x}>", msg.msgid, msg.to_msg_index());
        print!("  Author     : {}", author);
     msg
    };

    println!("  Author channel address: {}", author.channel_address().unwrap());

    println!("\nCreate Subscriber A and B");
    let mut subscriber_a = Subscriber::new(&random_seed(64), client.clone());
    let mut subscriber_b = Subscriber::new(&random_seed(64), client);


    println!("\nReceive Announcement for [Subscriber A, Subscriber B] ");
    subscriber_a.receive_announcement(&announcement_link).await.unwrap();
    print!("  Subscriber A: {}", subscriber_a);
    subscriber_b.receive_announcement(&announcement_link).await.unwrap();
    print!("  subscriber B: {}", subscriber_b);


    println!("\nSubscriber A => Generate Subscription Address");
    let subscribe_a_link = {
        let msg = subscriber_a.send_subscribe(&announcement_link).await.unwrap();
        println!("  msg => <{}> <{:x}>", msg.msgid, msg.to_msg_index());
        print!("  Subscriber A: {}", subscriber_a);
        msg
    };

    println!("\nAuthor Register Subscriber A");
    {
        author.receive_subscribe(&subscribe_a_link).await.unwrap();
        print!("  Author     : {}", author);
    }

    println!("\nAuthor Share keyload for everyone [Subscriber A]");
    let previous_msg_link = {
        let (msg, _seq) = author.send_keyload_for_everyone(&announcement_link).await.unwrap();
        println!("  msg => <{}> <{:x}>", msg.msgid, msg.to_msg_index());
        print!("  Author     : {}", author);
        msg
    };

    println!("\nReceive Keyload Subscriber A");
    subscriber_a.receive_keyload(&previous_msg_link).await.unwrap();

    println!("\nMessages public and masked");
    let public_payload = Bytes("PUBLICPAYLOAD1".as_bytes().to_vec());
    let masked_payload = Bytes("MASKEDPAYLOAD1".as_bytes().to_vec());

    println!("\nAuthor Send Signed packet 1");
    let _previous_msg_link = {
        let (msg, _seq) = author
            .send_signed_packet(&previous_msg_link, &public_payload, &masked_payload)
            .await.unwrap();
        println!("  msg => <{}> <{:x}>", msg.msgid, msg.to_msg_index());
        print!("  Author     : {}", author);
        msg
    };

    println!("\nSubscriber A fetching transactions...");
    utils::fetch_messages::s_fetch_next_messages(&mut subscriber_a).await;
    println!("\nSubscriber B fetching transactions...");
    utils::fetch_messages::s_fetch_next_messages(&mut subscriber_b).await;

    println!("\nSubscriber B => Generate Subscription Address");
    let subscribe_b_link = {
        let msg = subscriber_b.send_subscribe(&announcement_link).await.unwrap();
        println!("  msg => <{}> <{:x}>", msg.msgid, msg.to_msg_index());
        print!("  SubscriberB: {}", subscriber_a);
        msg
    };

    println!("\nAuthor Register Subscriber B");
    {
        author.receive_subscribe(&subscribe_b_link).await.unwrap();
        print!("  Author     : {}", author);
    }


    println!("\nAuthor sending new keyload to all subscribers [Subscriber A, Subscriber B]");
    let previous_msg_link2 = {
        let (msg, _seq) = author.send_keyload_for_everyone(&announcement_link).await.unwrap();
        println!("  msg => <{}> <{:x}>", msg.msgid, msg.to_msg_index());
        print!("  Author     : {}", author);
        msg
    };

    println!("\nReceive Keyload for [Subscriber A, Subscriber B]");
    subscriber_a.receive_keyload(&previous_msg_link2).await.unwrap();
    subscriber_b.receive_keyload(&previous_msg_link2).await.unwrap();

    println!("\nMessages 2 public and masked");
    let public_payload = Bytes("PUBLICPAYLOAD2".as_bytes().to_vec());
    let masked_payload = Bytes("MASKEDPAYLOAD2".as_bytes().to_vec());

    println!("\nAuthor Send Signed packet 2");
    let previous_msg_link2 = {
        let (msg, _seq) = author
            .send_signed_packet(&previous_msg_link2, &public_payload, &masked_payload)
            .await.unwrap();
        println!("  msg => <{}> <{:x}>", msg.msgid, msg.to_msg_index());
        print!("  Author     : {}", author);
        msg
    };
    

    println!("\nSubscriber A fetching transactions 2...");
    utils::fetch_messages::s_fetch_next_messages(&mut subscriber_a).await;

    println!("\nSubscriber B fetching transactions 2...");
    utils::fetch_messages::s_fetch_next_messages(&mut subscriber_b).await;

    println!("\nMessages 3 public and masked");
    let public_payload = Bytes("PUBLICPAYLOAD3".as_bytes().to_vec());
    let masked_payload = Bytes("MASKEDPAYLOAD3".as_bytes().to_vec());

    println!("\nAuthor Send Signed packet 3");
    let _previous_msg_link3 = {
        let (msg, _seq) = author
            .send_signed_packet(&previous_msg_link2, &public_payload, &masked_payload)
            .await.unwrap();
        println!("  msg => <{}> <{:x}>", msg.msgid, msg.to_msg_index());
        print!("  Author     : {}", author);
        msg
    };

    println!("\nSubscriber A fetching transactions 3...");
    utils::fetch_messages::s_fetch_next_messages(&mut subscriber_a).await;

    println!("\nSubscriber B fetching transactions 3...");
    utils::fetch_messages::s_fetch_next_messages(&mut subscriber_b).await;


}
