pub fn use_enum_associated_value() {
    println!("\n-------------------Enum Associated Value Usages-------------------------\n");
    
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("[::1]"));

    println!("Home ipv4 localhost is {}", home.read_value());
    println!("Home ipv6 localhost is {}", loopback.read_value());
}

enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn read_value(&self) -> String {
        match self {
            IpAddr::V4(v) => v.to_string(),
            IpAddr::V6(v) => v.to_string(),
        }
    }
}