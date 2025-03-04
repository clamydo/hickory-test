use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};
// Include the SHA1 hashes that were calculated at build time
const COMBINED_HASH: &str = include_str!(concat!(env!("OUT_DIR"), "/combined_hash.txt"));

#[tokio::main]
async fn main() {
    // Create default ResolverConfig
    let config = ResolverConfig::default();
    // Print the SHA1 hashes
    println!("Source file SHA1 hashes:");
    println!("  Combined (XOR): {}", COMBINED_HASH);

    // Configure options to force UDP
    let mut opts = ResolverOpts::default();
    opts.try_tcp_on_error = false;

    // Create resolver with custom config and options
    let resolver = TokioAsyncResolver::tokio(config, opts);
    let response = resolver.tlsa_lookup("_25._tcp.mx01.posteo.de.").await;

    println!("DNS Response: {:?}", response);
}
