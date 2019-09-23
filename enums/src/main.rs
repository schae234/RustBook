#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(id_kind: IpAddrKind) {
    println!("Routing {:?}!",id_kind)
}

#[allow(unused_variables)]
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
}
