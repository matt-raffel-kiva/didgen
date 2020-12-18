#![allow(unused_must_use)]

#[macro_use] extern crate clap;
extern crate rust_base58;
extern crate sodiumoxide;

use clap::{App, ArgMatches};
use rust_base58::ToBase58;
use sodiumoxide::crypto::sign::ed25519::{
    gen_keypair,
    keypair_from_seed,
    Seed
};

fn main() {
    // do all the configuration setup
    let yaml = load_yaml!("config.yml");
    let options: ArgMatches = App::from_yaml(yaml).get_matches();
    let seed_input: &str = options.value_of("seed").unwrap_or("tbd");
    let did: String;
    let ver_key: String;
    let private_key: String;
    match seed_input {
        "tbd" => {
            // testing if we can use sodiumoxide crate instead of indy
            let (v, s) = gen_keypair();
            did = v[0..16].to_vec().to_base58();
            ver_key = v[..].to_base58();
            private_key = s[..].to_base58();

        },
        _ => {
            // testing if we can use sodiumoxide crate instead of indy
            let seed_bytes: &[u8] = seed_input.clone().as_bytes();
            let (v, s) = keypair_from_seed(&Seed::from_slice(seed_bytes).unwrap());
            did = v[0..16].to_vec().to_base58();
            ver_key = v[..].to_base58();
            private_key = s[..].to_base58();
        }
    };

    println!("did          -> {}", did);
    println!("verkey       -> {}", ver_key);
    println!("pk           -> {}", private_key);

}
