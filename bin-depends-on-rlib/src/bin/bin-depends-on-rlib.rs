
extern crate rand;
extern crate rlib_depends_on_rust_openssl;
extern crate time;

use rlib_depends_on_rust_openssl::rlib::openssl::{OpenSSLCrypto};

fn main() {
    //let cipher = OpenSSLCrypto::new(&key[..], &iv[..]);
    let mut cipher = OpenSSLCrypto::new();

    //data: &[u8];
    //out: &mut BufMut;
    let mut message: Vec<u8> = Vec::new();
    for _ in 0..1024*1024 {
        message.push(rand::random::<u8>());
    }
    let data: &[u8] = &message;
    let mut out = Vec::new();

    let start_time = time::get_time();
    let start_time_millis = (start_time.sec as i64 * 1000) + (start_time.nsec as i64 / 1000 / 1000);

    assert!(cipher.update(data, &mut out).is_ok());
    assert!(cipher.finalize(&mut out).is_ok());

    let end_time = time::get_time();
    let end_time_millis = (end_time.sec as i64 * 1000) + (end_time.nsec as i64 / 1000 / 1000);

    let time_cost_millis = end_time_millis - start_time_millis;
    println!("time_cost_millis: {}", time_cost_millis);
}
