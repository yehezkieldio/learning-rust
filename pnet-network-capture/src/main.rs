use ctrlc;
use pnet::{
    datalink::{self, NetworkInterface},
    packet::ethernet::{EtherTypes, EthernetPacket},
};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::SystemTime;

fn get_interface() -> Option<NetworkInterface> {
    datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == "wlan0" && iface.is_up())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("\nShutting down...");
    })?;

    // Get network interface
    let interface = get_interface().ok_or("No suitable network interface found")?;
    println!("Capturing on interface: {}", interface.name);

    // Create channel
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => return Err("Unhandled channel type".into()),
        Err(e) => return Err(format!("Failed to create channel: {}", e).into()),
    };

    while running.load(Ordering::SeqCst) {
        match rx.next() {
            Ok(packet) => {
                if let Some(packet) = EthernetPacket::new(packet) {
                    let timestamp = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    // Only print IP packets
                    if packet.get_ethertype() == EtherTypes::Ipv4
                        || packet.get_ethertype() == EtherTypes::Ipv6
                    {
                        println!(
                            "[{}] Source: {}, Dest: {}, Type: {:?}",
                            timestamp,
                            packet.get_source(),
                            packet.get_destination(),
                            packet.get_ethertype()
                        );
                    }
                }
            }
            Err(e) => eprintln!("Error receiving packet: {}", e),
        }
    }

    Ok(())
}
