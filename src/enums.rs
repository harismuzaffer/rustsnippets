#[derive(Debug)]
struct IpV4Addr (i32, i32, i32, i32);

#[derive(Debug)]
struct Ipv6Addr {
    addr_part1: String
}

enum IpAddr {
    Ipv4(IpV4Addr),
    Ipv6(Ipv6Addr)
}

fn get_location(ipaddr: IpAddr) -> String{
    match ipaddr {
        IpAddr::Ipv4(mytuple) => format!("It is an IP V4 and address is {:?}", mytuple),
        IpAddr::Ipv6(addr) => addr.addr_part1,
    }
}

pub fn enums_with_struct() {
    let myipadd: IpAddr = IpAddr::Ipv4(IpV4Addr(2, 3, 4, 5));
    let mut x:String; 
    x = get_location(myipadd);
    
    let myipadd: IpAddr = IpAddr::Ipv6(Ipv6Addr{addr_part1: String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334")});
    x = get_location(myipadd);
    println!("It is an IP V6 and address is: {} ",x);
}
