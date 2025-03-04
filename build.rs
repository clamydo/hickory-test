use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use sha1::{Digest, Sha1};

fn main() {
    // Tell Cargo to re-run this if the files change
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");

    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Calculate SHA1 for main.rs
    let main_rs_path = Path::new("src/main.rs");
    let main_rs_digest = if let Ok(main_rs_content) = fs::read(main_rs_path) {
        let mut hasher = Sha1::new();
        hasher.update(&main_rs_content);
        let hash = hasher.finalize();
        
        // Write individual hash to a file
        let hash_str = format!("{:x}", hash);
        let dest_path = Path::new(&out_dir).join("main_rs_hash.txt");
        let mut f = File::create(&dest_path).unwrap();
        f.write_all(hash_str.as_bytes()).unwrap();
        
        hash.as_slice().to_vec()
    } else {
        vec![0; 20] // SHA1 is 20 bytes
    };

    // Calculate SHA1 for Cargo.lock
    let cargo_lock_path = Path::new("Cargo.lock");
    let cargo_lock_digest = if let Ok(cargo_lock_content) = fs::read(cargo_lock_path) {
        let mut hasher = Sha1::new();
        hasher.update(&cargo_lock_content);
        let hash = hasher.finalize();
        
        // Write individual hash to a file
        let hash_str = format!("{:x}", hash);
        let dest_path = Path::new(&out_dir).join("cargo_lock_hash.txt");
        let mut f = File::create(&dest_path).unwrap();
        f.write_all(hash_str.as_bytes()).unwrap();
        
        hash.as_slice().to_vec()
    } else {
        vec![0; 20] // SHA1 is 20 bytes
    };

    // Combine hashes using XOR
    let mut combined_hash = vec![0u8; 20];
    for i in 0..20 {
        combined_hash[i] = main_rs_digest[i] ^ cargo_lock_digest[i];
    }
    
    // Write combined hash to a file
    let combined_hash_str = combined_hash.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    
    let dest_path = Path::new(&out_dir).join("combined_hash.txt");
    let mut f = File::create(&dest_path).unwrap();
    f.write_all(combined_hash_str.as_bytes()).unwrap();
}