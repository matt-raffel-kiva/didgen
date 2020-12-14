#![allow(unused_must_use)]

#[macro_use] extern crate clap;
extern crate futures;
extern crate indyrs as indy;
#[macro_use] extern crate serde_json;

use clap::{App, ArgMatches};
use futures::future::Future;
use indy::{
    did,
    wallet,
    WalletHandle
};


fn main() {
    let yaml = load_yaml!("config.yml");
    let options: ArgMatches = App::from_yaml(yaml).get_matches();

    let wallet_name: &str = options.value_of("wallet").unwrap_or("didgen1.0");
    let wallet_password: &str= options.value_of("password").unwrap_or("9DXvkIMD7iSgD&RT$XYjHo0t");
    let config: String =  json!({
            "id": wallet_name,
            "storage_type": "default",
        }).to_string();
    let wallet_config: String = json!({
        "key" : wallet_password,
    }).to_string();


    // for now, ignore result :(
    let _result = wallet::create_wallet(&config, &wallet_config.clone()).wait();
    let handle: WalletHandle = wallet::open_wallet(&config, &wallet_config).wait().unwrap();

    let seed_input: &str = options.value_of("seed").unwrap_or("tbd");
    let mut did_json: String = "{}".to_owned();
    if seed_input != "tbd" {
        let json_config = json!({
           "seed": seed_input
        }).to_string();

        did_json = json_config.to_string();
    }

    let (did, verkey) = did::create_and_store_my_did(handle, &did_json).wait().unwrap();
    println!("did    -> {}", did);
    println!("verkey -> {}", verkey);

    wallet::close_wallet(handle).wait();
}
