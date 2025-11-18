use actix_web::{Responder, web};
use pnet::{datalink::{self, NetworkInterface}, ipnetwork::Ipv4Network, packet::ethernet::EthernetPacket};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::Packet;
use pnet::packet::FromPacket;

use crate::arp_scan;

pub async fn get_interfaces() -> impl Responder {
    let interfaces = datalink::interfaces();
    let names = interfaces.iter().map(|interface| interface.name.clone()).collect::<Vec<String>>();
    web::Json(names)

    /*
    for interface in interfaces {
        let cidr: Ipv4Network = get_network_addr(&interface);
        arp_scan(&interface, &cidr, timeout);
        let handle = thread::spawn(move || {
            capture_packets(&interface);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    */
}

pub fn get_interface(name: String) -> Option<NetworkInterface> {
    return datalink::interfaces().iter().find(|interface| interface.name == name).cloned();
}


pub fn capture_interface_packets(interface: &NetworkInterface) {
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled interface"),
        Err(e) => panic!("An error ocurred when creating a datalink channel: {}", e)
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    println!("New packet on {}", interface.name);
                    println!("Src: {} => Dest: {}: {}",
                        ethernet_packet.get_source(),
                        ethernet_packet.get_destination(),
                        ethernet_packet.get_ethertype());
                    let packet = ethernet_packet.packet();
                    let payload = ethernet_packet.payload();
                    let from_packet = ethernet_packet.from_packet();
                    //println!("---");
                    println!("packet: {:?}", packet);
                    // print the full packet as an array of u8
                    println!("payload: {:?}", payload);
                    // print the payload as an array of u8
                    println!("from_packet: {:?}", from_packet);
                    // print the hearder infos: mac address, ethertype, ...
                    // and the payload as an array of u8
                    println!("---");
                    
                }
            }
            Err(e)=> {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}

pub fn get_network_addr(interface: &NetworkInterface) -> Ipv4Network {
    let interface_ip = arp_scan::get_interface_ip(&interface);
    let ip_str = interface_ip.to_string();
    let network_parts = ip_str.split('.').map(|p| String::from(p)).collect::<Vec<String>>();
    let network_addr: String = format!("{}.0/24", network_parts[0..3].join("."));
    network_addr.parse().expect("Failed to get network address of interface")
}