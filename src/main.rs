/*
    This is an example implementation of a caesar cipher in Rust.
*/

extern crate argparse;
use argparse::{ArgumentParser, Store, StoreTrue};
use std::str;

fn main() {
    let mut key: u8 = 0;
    let mut text = String::new();
    let mut encode = false;
    let mut decode = false;

    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Encode in the way of the Romans !");
        parser.refer(&mut key).add_option(
            &["--key"],
            Store,
            "The text will be encrypted/decrypted with this specified key",
        );
        parser.refer(&mut text).add_option(
            &["--text"],
            Store,
            "The text you want to encrypt/decrypt",
        );
        parser
            .refer(&mut decode)
            .add_option(&["--dec"], StoreTrue, "Set if you want to decode");
        parser
            .refer(&mut encode)
            .add_option(&["--enc"], StoreTrue, "Set if you want to encode");

        parser.parse_args_or_exit();
    };

    if encode {
        println!("{}", caesar(&(text.to_uppercase()), key, true));
    } else if decode {
        println!("{}", caesar(&(text.to_uppercase()), key, false));
    };
}

fn caesar(plaintext: &String, key: u8, encode: bool) -> String {
    let mut return_bytearray = Vec::new();
    for character in plaintext.as_bytes() {
        if *character > 65 && *character < 90 {
            if encode {
                return_bytearray.push(((*character % 65) + (key % 26)) % 26 + 65);
            } else {
                return_bytearray.push(((*character % 65) + 26 - (key % 26)) % 26 + 65);
            };
        } else {
            return_bytearray.push(*character);
        };
    }
    match str::from_utf8(&return_bytearray) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Something went wrong ! {}", e),
    }
}
