
#[derive(Debug)]
struct IpV4Addr (i32,i32,i32,i32);

#[derive(Debug)]
struct Ipv6Addr {
    addr_part1: String
}

enum IpAddr {
    NonIpAddr(i32,i32,i32,i32),
    Ipv4(IpV4Addr),
    Ipv6(Ipv6Addr)
}

fn get_location(ipaddr: IpAddr) -> String{

    match ipaddr {
        IpAddr::Ipv4(mytuple) => format!("IP V4 string {:?}", mytuple),
        IpAddr::Ipv6(addr) => addr.addr_part1,
        IpAddr::NonIpAddr(v1,v2,v3,v4) => format!("IP V4 string {:?}", (v1,v2,v3,v4))
    }
}
fn main() {

    let myipadd: IpAddr = IpAddr::Ipv4(IpV4Addr(2,3,4,5));
    
    let mut x:String; 
    x = get_location(myipadd);
    println!("Output from function is: {} ",x);

    let myipadd: IpAddr = IpAddr::Ipv6(Ipv6Addr{addr_part1: String::from("An IPV6 Address")});

    x = get_location(myipadd);
    println!("Output from function is: {} ",x);


    let myipadd: IpAddr = IpAddr::NonIpAddr(2,3,4,5);
    x = get_location(myipadd);
    println!("Output from function is: {} ",x);

}
