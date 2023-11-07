#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddressKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddressKind::V4,
        address: String::from("192.168.18.2"),
    };
    let loopback = IpAddr {
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    };

    dbg!(home);
    dbg!(loopback);
}
