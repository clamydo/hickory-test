use hickory_resolver::{system_conf::read_system_conf, TokioAsyncResolver};
use hickory_resolver::{AsyncResolver, Name};

// Include the SHA1 hashes that were calculated at build time
const COMBINED_HASH: &str = include_str!(concat!(env!("OUT_DIR"), "/combined_hash.txt"));

#[tokio::main]
async fn main() {
    // Create default ResolverConfig and ResolverOpts
    let (config, mut opts) = read_system_conf().unwrap();

    // Configure options to force UDP
    opts.try_tcp_on_error = false;
    opts.cache_size = 0;
    opts.validate = true;

    // uncomment to reproduce bug in hichory where edns0 info in /etc/resolv.conf is not picked up
    opts.edns0 = true;

    // Create resolver with custom config and options
    let resolver = AsyncResolver::tokio(config, opts);
    let response = resolver
        .tlsa_lookup(Name::from_str_relaxed("_25._tcp.mx01.posteo.de.").expect("name"))
        .await
        .expect("lookup");

    println!("DNS Response: {:?}", response);
}
