/**
 * Implement and break HMAC-SHA1 with an artificial timing leak
 * The psuedocode on Wikipedia should be enough. HMAC is very easy.
 *
 * Using the web framework of your choosing (Sinatra, web.py, whatever), write a tiny application
 * that has a URL that takes a "file" argument and a "signature" argument, like so:
 *
 * http://localhost:10000/test?file=foo&signature=46b4ec586117154dacd49d664e5d63fdc88efb51
 * Have the server generate an HMAC key, and then verify that the "signature" on incoming requests
 * is valid for "file", using the "==" operator to compare the valid MAC for a file with the
 * "signature" parameter (in other words, verify the HMAC the way any normal programmer would
 * verify it).
 *
 * Write a function, call it "insecure_compare", that implements the == operation by doing
 * byte-at-a-time comparisons with early exit (ie, return false at the first non-matching byte).
 *
 * In the loop for "insecure_compare", add a 50ms sleep (sleep 50ms after each byte).
 *
 * Use your "insecure_compare" function to verify the HMACs on incoming requests, and test that the
 * whole contraption works. Return a 500 if the MAC is invalid, and a 200 if it's OK.
 *
 * Using the timing leak in this application, write a program that discovers the valid MAC for any
 * file.
 *
 * Why artificial delays?
 * Early-exit string compares are probably the most common source of cryptographic timing leaks,
 * but they aren't especially easy to exploit. In fact, many timing leaks (for instance, any in C,
 * C++, Ruby, or Python) probably aren't exploitable over a wide-area network at all. To play with
 * attacking real-world timing leaks, you have to start writing low-level timing code. We're
 * keeping things cryptographic in these challenges.
 */

use hyper::Client;
use hyper::Ok;
use time;

pub fn run() {
    // the URL I'm using is slightly different:
    // http://localhost:10000/slow?sleep=50&file=foo&signature=...
    solve(50, 2);
}

pub fn solve(sleep: u32, iter: u32) {
    let client = Client::new();
    let mut signature = [b'.'; 40];
    let chars = "0123456789abcdef".as_bytes();
    'outer: for i in 0..40 {
        let mut max = (0, 0);
        for j in 0..chars.len() {
            signature[i] = chars[j];
            let (t, r) = get(&client, &signature, sleep, iter);
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

fn get(client: &Client, signature: &[u8], sleep: u32, iter: u32) -> (u64, bool) {
    let s: String = signature.iter().map(|c| *c as char).collect();
    let u = format!("http://localhost:10000/slow?sleep={}&file=foo&signature={}",
                    sleep,
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
