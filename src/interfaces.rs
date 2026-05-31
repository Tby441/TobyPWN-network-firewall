use network_interface::{NetworkInterface, NetworkInterfaceConfig};

pub fn interfaces() {
    let interfaces: Vec<NetworkInterface> = NetworkInterface::show().unwrap();

    for interface in interfaces {
        println!("{:?}", interface.addr);
    }    
}