fn main() {

    #[derive(Debug)]
    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    let home: IpAddr = IpAddr::V4(127,0,0,1);
    println!("Home is {:?}", home);

    let loopback: IpAddr = IpAddr::V6(String::from("::1"));
    println!("Loopback is {:?}", loopback);
}
