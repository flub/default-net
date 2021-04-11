use default_net;

fn main(){
    if let Some(default_interface) = default_net::get_default_interface(){
        println!("Index {}", default_interface.index);
        println!("Name {}", default_interface.name);
        println!("MAC {:?}", default_interface.mac);
        println!("IPv4 {:?}", default_interface.ipv4);
        println!("IPv6 {:?}", default_interface.ipv6);
        println!("Gateway IP {:?}", default_interface.gateway.ip);
        println!("Gateway MAC {:?}", default_interface.gateway.mac);
    }else{
        println!("Failed to get default interface info");
    }
}