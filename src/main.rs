use hickory_resolver::Resolver;

fn main() {
    let resolver = Resolver::from_system_conf().unwrap();
    let response = resolver.tlsa_lookup("_25._tcp.mx01.posteo.de.");

    println!("Response: {:?}", response);
}
