#[derive(Debug)]
struct IpV4(u8,u8,u8,u8);

#[derive(Debug)]
struct IpV6(String);

#[derive(Debug)]
enum IpAddress {
    V4(IpV4),
    V6(IpV6),
}


fn main() {
    let home = IpAddress::V4(IpV4(127,0,0,1));
    let loopback = IpAddress::V6(IpV6(String::from("::01")));

    dbg!(&home);
    dbg!(&loopback);
}
