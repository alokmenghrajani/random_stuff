/**
 * Break fixed-nonce CTR mode using substitutions
 * Take your CTR encrypt/decrypt function and fix its nonce value to 0. Generate a random AES key.
 *
 * In successive encryptions (not in one big running CTR stream), encrypt each line of the base64
 * decodes of the following, producing multiple independent ciphertexts:
 *
 * SSBoYXZlIG1ldCB0aGVtIGF0IGNsb3NlIG9mIGRheQ==
 * Q29taW5nIHdpdGggdml2aWQgZmFjZXM=
 * RnJvbSBjb3VudGVyIG9yIGRlc2sgYW1vbmcgZ3JleQ==
 * RWlnaHRlZW50aC1jZW50dXJ5IGhvdXNlcy4=
 * SSBoYXZlIHBhc3NlZCB3aXRoIGEgbm9kIG9mIHRoZSBoZWFk
 * T3IgcG9saXRlIG1lYW5pbmdsZXNzIHdvcmRzLA==
 * T3IgaGF2ZSBsaW5nZXJlZCBhd2hpbGUgYW5kIHNhaWQ=
 * UG9saXRlIG1lYW5pbmdsZXNzIHdvcmRzLA==
 * QW5kIHRob3VnaHQgYmVmb3JlIEkgaGFkIGRvbmU=
 * T2YgYSBtb2NraW5nIHRhbGUgb3IgYSBnaWJl
 * VG8gcGxlYXNlIGEgY29tcGFuaW9u
 * QXJvdW5kIHRoZSBmaXJlIGF0IHRoZSBjbHViLA==
 * QmVpbmcgY2VydGFpbiB0aGF0IHRoZXkgYW5kIEk=
 * QnV0IGxpdmVkIHdoZXJlIG1vdGxleSBpcyB3b3JuOg==
 * QWxsIGNoYW5nZWQsIGNoYW5nZWQgdXR0ZXJseTo=
 * QSB0ZXJyaWJsZSBiZWF1dHkgaXMgYm9ybi4=
 * VGhhdCB3b21hbidzIGRheXMgd2VyZSBzcGVudA==
 * SW4gaWdub3JhbnQgZ29vZCB3aWxsLA==
 * SGVyIG5pZ2h0cyBpbiBhcmd1bWVudA==
 * VW50aWwgaGVyIHZvaWNlIGdyZXcgc2hyaWxsLg==
 * V2hhdCB2b2ljZSBtb3JlIHN3ZWV0IHRoYW4gaGVycw==
 * V2hlbiB5b3VuZyBhbmQgYmVhdXRpZnVsLA==
 * U2hlIHJvZGUgdG8gaGFycmllcnM/
 * VGhpcyBtYW4gaGFkIGtlcHQgYSBzY2hvb2w=
 * QW5kIHJvZGUgb3VyIHdpbmdlZCBob3JzZS4=
 * VGhpcyBvdGhlciBoaXMgaGVscGVyIGFuZCBmcmllbmQ=
 * V2FzIGNvbWluZyBpbnRvIGhpcyBmb3JjZTs=
 * SGUgbWlnaHQgaGF2ZSB3b24gZmFtZSBpbiB0aGUgZW5kLA==
 * U28gc2Vuc2l0aXZlIGhpcyBuYXR1cmUgc2VlbWVkLA==
 * U28gZGFyaW5nIGFuZCBzd2VldCBoaXMgdGhvdWdodC4=
 * VGhpcyBvdGhlciBtYW4gSSBoYWQgZHJlYW1lZA==
 * QSBkcnVua2VuLCB2YWluLWdsb3Jpb3VzIGxvdXQu
 * SGUgaGFkIGRvbmUgbW9zdCBiaXR0ZXIgd3Jvbmc=
 * VG8gc29tZSB3aG8gYXJlIG5lYXIgbXkgaGVhcnQs
 * WWV0IEkgbnVtYmVyIGhpbSBpbiB0aGUgc29uZzs=
 * SGUsIHRvbywgaGFzIHJlc2lnbmVkIGhpcyBwYXJ0
 * SW4gdGhlIGNhc3VhbCBjb21lZHk7
 * SGUsIHRvbywgaGFzIGJlZW4gY2hhbmdlZCBpbiBoaXMgdHVybiw=
 * VHJhbnNmb3JtZWQgdXR0ZXJseTo=
 * QSB0ZXJyaWJsZSBiZWF1dHkgaXMgYm9ybi4=
 * (This should produce 40 short CTR-encrypted ciphertexts).
 *
 * Because the CTR nonce wasn't randomized for each encryption, each ciphertext has been encrypted
 * against the same keystream. This is very bad.
 *
 * Understanding that, like most stream ciphers (including RC4, and obviously any block cipher run
 * in CTR mode), the actual "encryption" of a byte of data boils down to a single XOR operation, it
 * should be plain that:
 *
 * CIPHERTEXT-BYTE XOR PLAINTEXT-BYTE = KEYSTREAM-BYTE
 * And since the keystream is the same for every ciphertext:
 *
 * CIPHERTEXT-BYTE XOR KEYSTREAM-BYTE = PLAINTEXT-BYTE (ie, "you don't
 * say!")
 * Attack this cryptosystem piecemeal: guess letters, use expected English language frequence to
 * validate guesses, catch common English trigrams, and so on.
 *
 * Don't overthink it.
 * Points for automating this, but part of the reason I'm having you do this is that I think this
 * approach is suboptimal.
 */

use utils::base64::base64_decode;
use utils::aes::aes_ctr;
use utils::hex::hex_decode;

use set1::challenge3::ascii_score;

pub fn run() {
    let inputs = ["SSBoYXZlIG1ldCB0aGVtIGF0IGNsb3NlIG9mIGRheQ==",
                  "Q29taW5nIHdpdGggdml2aWQgZmFjZXM=",
                  "RnJvbSBjb3VudGVyIG9yIGRlc2sgYW1vbmcgZ3JleQ==",
                  "RWlnaHRlZW50aC1jZW50dXJ5IGhvdXNlcy4=",
                  "SSBoYXZlIHBhc3NlZCB3aXRoIGEgbm9kIG9mIHRoZSBoZWFk",
                  "T3IgcG9saXRlIG1lYW5pbmdsZXNzIHdvcmRzLA==",
                  "T3IgaGF2ZSBsaW5nZXJlZCBhd2hpbGUgYW5kIHNhaWQ=",
                  "UG9saXRlIG1lYW5pbmdsZXNzIHdvcmRzLA==",
                  "QW5kIHRob3VnaHQgYmVmb3JlIEkgaGFkIGRvbmU=",
                  "T2YgYSBtb2NraW5nIHRhbGUgb3IgYSBnaWJl",
                  "VG8gcGxlYXNlIGEgY29tcGFuaW9u",
                  "QXJvdW5kIHRoZSBmaXJlIGF0IHRoZSBjbHViLA==",
                  "QmVpbmcgY2VydGFpbiB0aGF0IHRoZXkgYW5kIEk=",
                  "QnV0IGxpdmVkIHdoZXJlIG1vdGxleSBpcyB3b3JuOg==",
                  "QWxsIGNoYW5nZWQsIGNoYW5nZWQgdXR0ZXJseTo=",
                  "QSB0ZXJyaWJsZSBiZWF1dHkgaXMgYm9ybi4=",
                  "VGhhdCB3b21hbidzIGRheXMgd2VyZSBzcGVudA==",
                  "SW4gaWdub3JhbnQgZ29vZCB3aWxsLA==",
                  "SGVyIG5pZ2h0cyBpbiBhcmd1bWVudA==",
                  "VW50aWwgaGVyIHZvaWNlIGdyZXcgc2hyaWxsLg==",
                  "V2hhdCB2b2ljZSBtb3JlIHN3ZWV0IHRoYW4gaGVycw==",
                  "V2hlbiB5b3VuZyBhbmQgYmVhdXRpZnVsLA==",
                  "U2hlIHJvZGUgdG8gaGFycmllcnM/",
                  "VGhpcyBtYW4gaGFkIGtlcHQgYSBzY2hvb2w=",
                  "QW5kIHJvZGUgb3VyIHdpbmdlZCBob3JzZS4=",
                  "VGhpcyBvdGhlciBoaXMgaGVscGVyIGFuZCBmcmllbmQ=",
                  "V2FzIGNvbWluZyBpbnRvIGhpcyBmb3JjZTs=",
                  "SGUgbWlnaHQgaGF2ZSB3b24gZmFtZSBpbiB0aGUgZW5kLA==",
                  "U28gc2Vuc2l0aXZlIGhpcyBuYXR1cmUgc2VlbWVkLA==",
                  "U28gZGFyaW5nIGFuZCBzd2VldCBoaXMgdGhvdWdodC4=",
                  "VGhpcyBvdGhlciBtYW4gSSBoYWQgZHJlYW1lZA==",
                  "QSBkcnVua2VuLCB2YWluLWdsb3Jpb3VzIGxvdXQu",
                  "SGUgaGFkIGRvbmUgbW9zdCBiaXR0ZXIgd3Jvbmc=",
                  "VG8gc29tZSB3aG8gYXJlIG5lYXIgbXkgaGVhcnQs",
                  "WWV0IEkgbnVtYmVyIGhpbSBpbiB0aGUgc29uZzs=",
                  "SGUsIHRvbywgaGFzIHJlc2lnbmVkIGhpcyBwYXJ0",
                  "SW4gdGhlIGNhc3VhbCBjb21lZHk7",
                  "SGUsIHRvbywgaGFzIGJlZW4gY2hhbmdlZCBpbiBoaXMgdHVybiw=",
                  "VHJhbnNmb3JtZWQgdXR0ZXJseTo=",
                  "QSB0ZXJyaWJsZSBiZWF1dHkgaXMgYm9ybi4="];
    let key = hex_decode("cf358337dd4dfc1ddb710e30a1809e3f");
    let nonce = [0; 8];
    let mut ciphertexts = Vec::with_capacity(inputs.len());
    let mut max_len = 0;
    let mut plaintexts = Vec::new();
    for input in inputs.iter() {
        let plaintext = base64_decode(input);
        plaintexts.push(String::from_utf8(plaintext).unwrap());
        let ciphertext = aes_ctr(&key, &nonce, &base64_decode(input));
        if ciphertext.len() > max_len {
            max_len = ciphertext.len();
        }
        ciphertexts.push(ciphertext);
    }

    // Break each character. We keep the first ciphertext which yields ascii in the
    // response. We'll re-use the scoring function from set1, challenge 3.
    for i in 0..max_len {
        let mut best = (0, Vec::new());
        for j in 0..256 {
            let mut v = Vec::new();
            for k in 0..ciphertexts.len() {
                if i >= ciphertexts[k].len() {
                    v.push(' ' as u8);
                    continue;
                }
                v.push(ciphertexts[k][i] ^ (j as u8));
            }
            let b = ascii_score(&v);
            if b > best.0 {
                best = (b, v);
            }
        }
        for k in 0..ciphertexts.len() {
            if i >= ciphertexts[k].len() {
                continue;
            }
            ciphertexts[k][i] = best.1[k];
        }
    }

    for i in 0..ciphertexts.len() {
        let t: String = ciphertexts[i].iter().map(|c| *c as char).collect();
        if plaintexts[i] == t {
            println!("ok {}", t);
        } else {
            println!("fail {}\n  vs {}", t, plaintexts[i]);
        }
    }
}
