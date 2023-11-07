#[derive(Debug)]
enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddressKind::V4(128, 25, 28, 3);
    let loopback = IpAddressKind::V6(String::from("::1"));

    dbg!(home);
    dbg!(loopback);
}


