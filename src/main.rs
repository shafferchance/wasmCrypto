extern crate hex;

//use aes_soft::Aes256;
use bcrypt;
use sha2::{ Sha256, Digest };
use std::str;
use std::u8;
//use wasm_bindgen::prelude::*;

fn calc_pw_hash (username: &str, pword: &str, buffer: &mut [u8]) {
    let mut hasher = Sha256::new();
    let hash_str = [username, pword].concat();
    hasher.input(hash_str);
    let pword_salt: Vec<u8> = hasher.result().iter().cloned().rev().take(16).collect();
    bcrypt::bcrypt(bcrypt::DEFAULT_COST, pword_salt.as_slice(), pword.as_bytes(), buffer);
}

pub fn get_pw_hash(username: &str, pword: &str) -> String {
    let mut buffer: [u8; 24] = [0; 24];
    calc_pw_hash(username, pword, &mut buffer);
    hex::encode(buffer)
}
/*
fn sha_hash(username: &str, pword: &str) {

}
*/

fn main() {
    let username = "person";
    let password = "h3!@##fdflk!";

    println!("{}", get_pw_hash(username, password));
}
