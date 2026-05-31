mod interfaces;

fn main() {
    println!("TobyPWN Firewall v0.0.1");

    println!("Scanning for interfaces");

    interfaces::interfaces();
}