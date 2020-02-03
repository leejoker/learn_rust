//定义枚举
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IPV4 {
    address: (u8, u8, u8, u8),
}

#[derive(Debug)]
struct IPV6 {
    address: String,
}

#[derive(Debug)]
enum IpAddrKindWithArg {
    V4(IPV4),
    V6(IPV6),
    OTHER,
    // V4(u8, u8, u8, u8),
    // V6(String),
}

impl IpAddrKindWithArg {
    fn show_ip(&self) {
        match self {
            IpAddrKindWithArg::V4(ipv4) => {
                println!(
                    "ip address is {}.{}.{}.{}",
                    ipv4.address.0, ipv4.address.1, ipv4.address.2, ipv4.address.3
                );
            }
            IpAddrKindWithArg::V6(ipv6) => {
                println!("ip address is {}", ipv6.address);
            }
            //增加通配符
            _ => println!("there is something wrong!"),
            // IpAddrKindWithArg::V4(arg1, arg2, arg3, arg4) => {
            //     println!("ip address is {}.{}.{}.{}", arg1, arg2, arg3, arg4);
            // }
            // IpAddrKindWithArg::V6(ipv6) => {
            //     println!("ip address is {}", ipv6);
            // }
        }
    }

    fn show_ip_use_if_let(&self) {
        if let IpAddrKindWithArg::V4(ipv4) = self {
            println!(
                "ip address is {}.{}.{}.{}",
                ipv4.address.0, ipv4.address.1, ipv4.address.2, ipv4.address.3
            );
        }
    }
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;
    let home = IpAddr {
        kind: v4,
        address: String::from("127.0.0.1"),
    };
    println!("home ip address is {:#?}", home);
    let loopback = IpAddr {
        kind: v6,
        address: String::from("::1"),
    };
    println!("loopback ip address is {:#?}", loopback);

    let home_change = IpAddrKindWithArg::V4(IPV4 {
        address: (127, 0, 0, 1),
    });
    let loopback_change = IpAddrKindWithArg::V6(IPV6 {
        address: String::from("::1"),
    });
    let other = IpAddrKindWithArg::OTHER;
    // let home_change = IpAddrKindWithArg::V4(127, 0, 0, 1);
    // let loopback_change = IpAddrKindWithArg::V6(String::from("::1"));
    println!("{:#?}, {:#?}, {:#?}", home_change, loopback_change, other);
    home_change.show_ip();
    home_change.show_ip_use_if_let();
}
