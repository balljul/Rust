#[derive(Debug)]
enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddressKind {
    fn print(&self) {
        println!("The Ip-Address is: {:?}", self)
    }
}

fn main() {
    let home = IpAddressKind::V4(128, 25, 28, 3);
    let loopback = IpAddressKind::V6(String::from("::1"));
    loopback.print();
    home.print();
}


