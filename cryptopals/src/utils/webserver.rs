use iron::prelude::*;
use iron::status;
use params::Params;
use params::Value;
use router::Router;
use utils::sha1::Sha1;
use utils::hex::hex_encode;
use utils::hex::hex_decode;
use std::thread;
use std::time;

pub fn start(port: u32) {
    // make sure hmac_sha1 is correct
    let r = hmac_sha1(&[], &[]);
    assert_eq!(hex_encode(&r), "fbdb1d1b18aa6c08324b7d64b71fb76370690e1d");

    println!("starting webserver on localhost:{}", port);

    let mut router = Router::new();
    router.get("/slow", handle_slow, "index");        // let router = router!(index: get "/" => handler,

    Iron::new(router).http(format!("localhost:{}", port)).unwrap();
}

fn handle_slow(req: &mut Request) -> IronResult<Response> {
    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");

    let sleep: u64 = get_param(req, "sleep").parse().unwrap_or(50);
    let file = get_param(req, "file");
    let signature = get_param(req, "signature");

    let expected = hex_encode(&hmac_sha1(&key, file.as_bytes()));
    println!("sleep={}, file={}, signature={}\n                     expected={}",
             sleep,
             file,
             signature,
             expected);

    if slow_compare(expected, signature, sleep) {
        return Ok(Response::with(status::Ok));
    } else {
        return Ok(Response::with(status::InternalServerError));
    }
}

fn slow_compare(a: String, b: String, s: u64) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    if a_bytes.len() != b_bytes.len() {
        return false;
    }
    for i in 0..a_bytes.len() {
        if a_bytes[i] != b_bytes[i] {
            return false;
        }
        thread::sleep(time::Duration::from_millis(s));
    }
    return true;
}

fn get_param(req: &mut Request, name: &str) -> String {
    let params = req.get_ref::<Params>().unwrap();
    match params.find(&[name]) {
        Some(&Value::String(ref val)) => return val.clone(),
        _ => return String::from(""),
    }
}

fn hmac_sha1(key: &[u8], msg: &[u8]) -> Vec<u8> {
    let mut k = [0; 64];
    if key.len() > 64 {
        let t = sha1(key);
        for i in 0..20 {
            k[i] = t[i];
        }
    } else {
        for i in 0..key.len() {
            k[i] = key[i];
        }
    }
    let mut o_key_pad = [0; 64];
    for i in 0..64 {
        o_key_pad[i] = k[i] ^ 0x5c;
    }

    let mut i_key_pad = [0; 64];
    for i in 0..64 {
        i_key_pad[i] = k[i] ^ 0x36;
    }

    let mut inner_hash = Vec::new();
    inner_hash.extend_from_slice(&i_key_pad);
    inner_hash.extend_from_slice(msg);

    let mut outer_hash = Vec::new();
    outer_hash.extend_from_slice(&o_key_pad);
    outer_hash.extend(sha1(&inner_hash));

    return sha1(&outer_hash);
}

fn sha1(msg: &[u8]) -> Vec<u8> {
    let mut sha1 = Sha1::new();
    sha1.update(msg);
    let mut r = Vec::new();
    r.extend_from_slice(&sha1.digest().bytes());
    return r;
}
