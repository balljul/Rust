#[derive(Debug)]
enum IpAddressKind {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddressKind::V4(String::from("192.168.18.2"));
    let loopback = IpAddressKind::V6(String::from("::1"));

    dbg!(home);
    dbg!(loopback);
}
