#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddress {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddress1 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// An even better approach is to make different structs and use them as type for enum
#[derive(Debug)]
struct Ipv4Addr {
    part_1: u8,
    part_2: u8,
    part_3: u8,
    part_4: u8,
}

#[derive(Debug)]
struct Ipv6Addr {
    part_1: String,
}

#[derive(Debug)]
enum BestIpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:#?}", home);
    println!("{:#?}", loopback);

    let home = IpAddress::V4(String::from("127.0.0.1"));
    let loopback = IpAddress::V6(String::from("::1"));

    println!("{:#?}", home);
    println!("{:#?}", loopback);

    // Another advantage of using an enum with attached value over struct is every entry can have different values
    let home = IpAddress1::V4(127, 0, 0, 1);
    let loopback = IpAddress1::V6(String::from("::1"));

    println!("{:#?}", home);
    println!("{:#?}", loopback);

    // Best way - use structs for representing individual types
    let ipv4_addr = Ipv4Addr {
        part_1: 127,
        part_2: 0,
        part_3: 0,
        part_4: 1,
    };

    let ipv6_addr = Ipv6Addr {
        part_1: String::from("::1"),
    };

    let home = BestIpAddr::V4(ipv4_addr);
    let loopback = BestIpAddr::V6(ipv6_addr);

    println!("{:#?}", home);
    println!("{:#?}", loopback);
}

fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind);
}