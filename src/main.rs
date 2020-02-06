extern crate argparse;
use argparse::{ArgumentParser, Store, StoreTrue};

fn main() {
    let mut key: i8 = 0;
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

    let mut end_text = String::new();

    if encode {
        println!("{}", encode_caesar(&(text.to_uppercase()), key));
    } else if decode {
        println!("{}", decode_caesar(&(text.to_uppercase()), key));
    };
}

fn encode_caesar(plaintext: &String, key: i8) -> String {
    plaintext.to_string()
}

fn decode_caesar(ciphertext: &String, key: i8) -> String {
    ciphertext.to_string()
}
