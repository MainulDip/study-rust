pub fn use_enum_associated_value() {
    println!("\n-------------------Enum Associated Value Usages-------------------------\n");
    
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("[::1]"));

    println!("Home ipv4 localhost is {}", home.readValue());
    println!("Home ipv6 localhost is {}", loopback.readValue());
}

enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn readValue(&self) -> String {
        match self {
            IpAddr::V4(v) => v.to_string(),
            IpAddr::V6(v) => v.to_string(),
        }
    }
}