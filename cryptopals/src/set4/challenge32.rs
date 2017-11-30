/**
 * Break HMAC-SHA1 with a slightly less artificial timing leak
 * Reduce the sleep in your "insecure_compare" until your previous solution breaks. (Try 5ms to start.)
 *
 * Now break it again.
 */

use hyper::Client;
use hyper::Ok;
use time;

pub fn run() {
    // the URL I'm using is slightly different:
    // http://localhost:10000/slow?sleep=5&file=foo&signature=...
    let client = Client::new();
    let mut signature = [b'.'; 40];
    let chars = "0123456789abcdef".as_bytes();
    'outer: for i in 0..40 {
        let mut max = (0, 0);
        for j in 0..chars.len() {
            signature[i] = chars[j];
            let (t, r) = get(&client, &signature, 30);
            if r == true {
                println!("success!");
                break 'outer;
            }
            if t > max.0 {
                max = (t, chars[j]);
            }
        }
        signature[i] = max.1;
        let s: String = signature.iter().take(i + 1).map(|c| *c as char).collect();
        println!("making progress: {}", s);
    }
    let s: String = signature.iter().map(|c| *c as char).collect();
    println!("{}", s);
}

fn get(client: &Client, signature: &[u8], iter: u32) -> (u64, bool) {
    let s: String = signature.iter().map(|c| *c as char).collect();
    let u = format!("http://localhost:10000/slow?sleep=1&file=foo&signature={}",
                    s);
    let mut t = 0;
    for _ in 0..iter {
        let before = time::precise_time_ns();
        let res = client.get(&u)
            .send()
            .unwrap();
        if res.status == Ok {
            return (t, true);
        }
        let after = time::precise_time_ns();
        t += after - before;
    }
    return (t, false);
}
