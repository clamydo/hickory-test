use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};

#[tokio::main]
async fn main() {
    // Create default ResolverConfig
    let config = ResolverConfig::default();

    // Configure options to force UDP
    let mut opts = ResolverOpts::default();
    opts.try_tcp_on_error = false;

    // Create resolver with custom config and options
    let resolver = TokioAsyncResolver::tokio(config, opts);
    let response = resolver.tlsa_lookup("_25._tcp.mx01.posteo.de.").await;

    println!("DNS Response: {:?}", response);
}
