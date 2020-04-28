// I liked the book's IP example so I'm just annotating stuff here

// variants of the enum are just strings labelled v4 or v6
// this means that we could give two ips different behaviours despite them both being strings
// the same effect could be achieved by defining two structs IPv6 and IPv4, but then you couldn't
// use a single impl block to define shared behaviour 
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    // here you can define behaviour on IPs in general
}

fn do_stuff() {
    // home is a V4 variant of IpAddr that stores 127.0.0.1
    let home = IpAddr::V4(127, 0, 0, 1);
}
