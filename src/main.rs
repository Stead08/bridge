use std::env;
use pnet::datalink::{self, Channel::Ethernet, DataLinkReceiver, DataLinkSender, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;
use tokio::task;
use log::{debug, error};
use pnet::packet::Packet;

/**
* Bridge between two network interfaces.
*/
#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        error!("Please specify target interface name [target1] [target2]");
        std::process::exit(1);
    }
    let interface_name_1 = &args[1];
    let interface_name_2 = &args[2];

    let (interface_1, tx1, rx1) = get_interface_and_channel(interface_name_1);
    let (interface_2, tx2, rx2) = get_interface_and_channel(interface_name_2);

    // `interface_1`から`interface_2`への送信タスク
    let task1 = task::spawn(relay_packets(tx2, rx1, interface_2));

    // `interface_2`から`interface_1`への送信タスク
    let task2 = task::spawn(relay_packets(tx1, rx2, interface_1));

    // タスクが完了するのを待つ
    let _ = tokio::try_join!(task1, task2);
}

/**
* パケットを中継する
*/
async fn relay_packets(mut tx: Box<dyn DataLinkSender>, mut rx: Box<dyn DataLinkReceiver>, interface: NetworkInterface) {
    loop {
        match rx.next() {
            Ok(frame) => {
                let frame = EthernetPacket::new(frame).unwrap();
                let destination = &frame.get_destination();
                let source = &frame.get_source();
                let ether_type = frame.get_ethertype();
                let ether_type_hex = ether_type.0;

                debug!(
                    "Destination: {}, Source: {}, EtherType: {}({:#x})",
                    destination, source, ether_type, ether_type_hex,);

                tx.send_to(frame.packet(), Some(interface.clone()));
            }
            Err(e) => error!("Failed to read: {}", e),
        }
    }
}

/**
* 指定された名前のネットワークインターフェースを取得する
*/
fn get_interface_and_channel(name: &str) -> (NetworkInterface, Box<dyn DataLinkSender>, Box<dyn DataLinkReceiver>) {
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
        .find(|iface| iface.name == *name)
        .expect("Failed to get interface");

    let (tx, rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create datalink channel {}", e),
    };

    (interface, tx, rx)
}