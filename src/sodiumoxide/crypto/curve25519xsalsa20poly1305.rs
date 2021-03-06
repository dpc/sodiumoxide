/*!
`crypto_box_curve25519xsalsa20poly1305` , a particular
combination of Curve25519, Salsa20, and Poly1305 specified in
[Cryptography in NaCl](http://nacl.cr.yp.to/valid.html).

This function is conjectured to meet the standard notions of privacy and
third-party unforgeability.

*/
use libc::{c_ulonglong, c_int};
use utils::marshal;
use randombytes::randombytes_into;
use std::intrinsics::{transmute,copy_nonoverlapping_memory};
use std::fmt;

#[link(name = "sodium")]
extern {
    fn crypto_box_curve25519xsalsa20poly1305_keypair(pk: *mut u8,
                                                     sk: *mut u8) -> c_int;
    fn crypto_box_curve25519xsalsa20poly1305(c: *mut u8,
                                             m: *const u8,
                                             mlen: c_ulonglong,
                                             n: *const u8,
                                             pk: *const u8,
                                             sk: *const u8) -> c_int;
    fn crypto_box_curve25519xsalsa20poly1305_open(m: *mut u8,
                                                  c: *const u8,
                                                  clen: c_ulonglong,
                                                  n: *const u8,
                                                  pk: *const u8,
                                                  sk: *const u8) -> c_int;
    fn crypto_box_curve25519xsalsa20poly1305_beforenm(k: *mut u8,
                                                      pk: *const u8,
                                                      sk: *const u8) -> c_int;
    fn crypto_box_curve25519xsalsa20poly1305_afternm(c: *mut u8,
                                                     m: *const u8,
                                                     mlen: c_ulonglong,
                                                     n: *const u8,
                                                     k: *const u8) -> c_int;
    fn crypto_box_curve25519xsalsa20poly1305_open_afternm(m: *mut u8,
                                                          c: *const u8,
                                                          clen: c_ulonglong,
                                                          n: *const u8,
                                                          k: *const u8) -> c_int;
}

pub static PUBLICKEYBYTES: uint = 32;
pub static SECRETKEYBYTES: uint = 32;
pub static NONCEBYTES: uint = 24;
pub static PRECOMPUTEDKEYBYTES: uint = 32;
static ZEROBYTES: uint = 32;
static BOXZEROBYTES: uint = 16;

/**
 * `PublicKey` for asymmetric authenticated encryption
 */
pub struct PublicKey(pub [u8, ..PUBLICKEYBYTES]);


impl PublicKey {
    /// Create `PublicKey` from slice.
    pub fn from_slice(data: &[u8]) -> Result<PublicKey, ()> {
        match data.len() {
            PUBLICKEYBYTES => {
                let mut ret = [0, ..PUBLICKEYBYTES];

                unsafe {
                    copy_nonoverlapping_memory(ret.as_mut_ptr(),
                                               data.as_ptr(),
                                               data.len());
                }
                Ok(PublicKey(ret))
            },
            _ => Err(())
        }
    }

    /// Create `PublicKey` from slice without copying.
    pub fn from_slice_by_ref<'a>(data: &'a [u8]) -> Result<&'a PublicKey, ()> {
        match data.len() {
            PUBLICKEYBYTES => {
                Ok(unsafe {
                    transmute(data.as_ptr())
                })
            },
            _ => Err(())
        }
    }

    /// Borrow as a slice.
    pub fn as_slice(&self) -> &[u8] {
        let &PublicKey(ref data) = self;
        data.as_slice()
    }

}

impl PartialEq for PublicKey {
    fn eq(&self, other: &PublicKey) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for PublicKey {}

impl fmt::Show for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

/**
 * `SecretKey` for asymmetric authenticated encryption
 *
 * When a `SecretKey` goes out of scope its contents
 * will be zeroed out
 */
pub struct SecretKey(pub [u8, ..SECRETKEYBYTES]);

impl SecretKey {
    /// Create `SecretKey` from slice.
    pub fn from_slice(data: &[u8]) -> Result<SecretKey, ()> {
        match data.len() {
            SECRETKEYBYTES => {
                let mut ret = [0, ..SECRETKEYBYTES];

                unsafe {
                    copy_nonoverlapping_memory(ret.as_mut_ptr(),
                                               data.as_ptr(),
                                               data.len());
                }
                Ok(SecretKey(ret))
            },
            _ => Err(())
        }
    }

    /// Create `SecretKey` from slice without copying.
    pub fn from_slice_by_ref<'a>(data: &'a [u8]) -> Result<&'a SecretKey, ()> {
        match data.len() {
            SECRETKEYBYTES => {
                Ok(unsafe {
                    transmute(data.as_ptr())
                })
            },
            _ => Err(())
        }
    }

    /// Borrow as a slice.
    pub fn as_slice(&self) -> &[u8] {
        let &SecretKey(ref data) = self;
        data.as_slice()
    }
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        let &SecretKey(ref mut sk) = self;
        for e in sk.mut_iter() { *e = 0 }
    }
}

impl PartialEq for SecretKey {
    fn eq(&self, other: &SecretKey) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for SecretKey { }

/**
 * `Nonce` for asymmetric authenticated encryption
 */
pub struct Nonce(pub [u8, ..NONCEBYTES]);

impl Nonce {
    /// Create `Nonce` from slice.
    pub fn from_slice(data: &[u8]) -> Result<Nonce, ()> {
        match data.len() {
            NONCEBYTES => {
                let mut ret = [0, ..NONCEBYTES];

                unsafe {
                    copy_nonoverlapping_memory(ret.as_mut_ptr(),
                                               data.as_ptr(),
                                               data.len());
                }
                Ok(Nonce(ret))
            },
            _ => Err(())
        }
    }


    /// Create `Nonce` from slice without copying.
    pub fn from_slice_by_ref<'a>(data: &'a [u8]) -> Result<&'a Nonce, ()> {
        match data.len() {
            NONCEBYTES => {
                Ok(unsafe {
                    transmute(data.as_ptr())
                })
            },
            _ => Err(())
        }
    }

    /// Borrow as a slice.
    pub fn as_slice(&self) -> &[u8] {
        let &Nonce(ref data) = self;
        data.as_slice()
    }
}

impl PartialEq for Nonce {
    fn eq(&self, other: &Nonce) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for Nonce {}

impl fmt::Show for Nonce {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

/**
 * `gen_keypair()` randomly generates a secret key and a corresponding public key.
 *
 * THREAD SAFETY: `gen_keypair()` is thread-safe provided that you have
 * called `sodiumoxide::init()` once before using any other function
 * from sodiumoxide.
 */
pub fn gen_keypair() -> (PublicKey, SecretKey) {
    unsafe {
        let mut pk = [0u8, ..PUBLICKEYBYTES];
        let mut sk = [0u8, ..SECRETKEYBYTES];
        crypto_box_curve25519xsalsa20poly1305_keypair(
            pk.as_mut_ptr(),
            sk.as_mut_ptr());
        (PublicKey(pk), SecretKey(sk))
    }
}

/**
 * `gen_nonce()` randomly generates a nonce
 *
 * THREAD SAFETY: `gen_nonce()` is thread-safe provided that you have
 * called `sodiumoxide::init()` once before using any other function
 * from sodiumoxide.
 */
pub fn gen_nonce() -> Nonce {
    let mut n = [0, ..NONCEBYTES];
    randombytes_into(n);
    Nonce(n)
}

/**
 * `seal()` encrypts and authenticates a message `m` using the senders secret key `sk`,
 * the receivers public key `pk` and a nonce `n`. It returns a ciphertext `c`.
 */
pub fn seal(m: &[u8],
            &Nonce(n): &Nonce,
            &PublicKey(pk): &PublicKey,
            &SecretKey(sk): &SecretKey) -> Vec<u8> {
    let (c, _) = marshal(m, ZEROBYTES, BOXZEROBYTES, |dst, src, len| {
        unsafe {
            crypto_box_curve25519xsalsa20poly1305(dst,
                                                  src,
                                                  len,
                                                  n.as_ptr(),
                                                  pk.as_ptr(),
                                                  sk.as_ptr());
        }
    });
    c
}

/**
 * `open()` verifies and decrypts a ciphertext `c` using the receiver's secret key `sk`,
 * the senders public key `pk`, and a nonce `n`. It returns a plaintext `Some(m)`.
 * If the ciphertext fails verification, `open()` returns `None`.
 */
pub fn open(c: &[u8],
            &Nonce(n): &Nonce,
            &PublicKey(pk): &PublicKey,
            &SecretKey(sk): &SecretKey) -> Option<Vec<u8>> {
    if c.len() < BOXZEROBYTES {
        return None
    }
    let (m, ret) = marshal(c, BOXZEROBYTES, ZEROBYTES, |dst, src, len| {
        unsafe {
            crypto_box_curve25519xsalsa20poly1305_open(dst,
                                                       src,
                                                       len,
                                                       n.as_ptr(),
                                                       pk.as_ptr(),
                                                       sk.as_ptr())
        }
    });
    if ret == 0 {
        Some(m)
    } else {
        None
    }
}

/**
 * Applications that send several messages to the same receiver can gain speed by
 * splitting `seal()` into two steps, `precompute()` and `seal_precomputed()`.
 * Similarly, applications that receive several messages from the same sender can gain
 * speed by splitting `open()` into two steps, `precompute()` and `open_precomputed()`.
 *
 * When a `PrecomputedKey` goes out of scope its contents will be zeroed out
 */
pub struct PrecomputedKey([u8, ..PRECOMPUTEDKEYBYTES]);
impl Drop for PrecomputedKey {
    fn drop(&mut self) {
        let &PrecomputedKey(ref mut k) = self;
        for e in k.mut_iter() { *e = 0 }
    }
}

/**
 * `precompute()` computes an intermediate key that can be used by `seal_precomputed()`
 * and `open_precomputed()`
 */
pub fn precompute(&PublicKey(pk): &PublicKey,
                  &SecretKey(sk): &SecretKey) -> PrecomputedKey {
    let mut k = [0u8, ..PRECOMPUTEDKEYBYTES];
    unsafe {
        crypto_box_curve25519xsalsa20poly1305_beforenm(k.as_mut_ptr(),
                                                       pk.as_ptr(),
                                                       sk.as_ptr());
    }
    PrecomputedKey(k)
}

/**
 * `seal_precomputed()` encrypts and authenticates a message `m` using a precomputed key `k`,
 * and a nonce `n`. It returns a ciphertext `c`.
 */
pub fn seal_precomputed(m: &[u8],
                        &Nonce(n): &Nonce,
                        &PrecomputedKey(k): &PrecomputedKey) -> Vec<u8> {
    let (c, _) = marshal(m, ZEROBYTES, BOXZEROBYTES, |dst, src, len| {
        unsafe {
            crypto_box_curve25519xsalsa20poly1305_afternm(dst,
                                                          src,
                                                          len,
                                                          n.as_ptr(),
                                                          k.as_ptr());
        }
    });
    c
}

/**
 * `open_precomputed()` verifies and decrypts a ciphertext `c` using a precomputed
 * key `k` and a nonce `n`. It returns a plaintext `Some(m)`.
 * If the ciphertext fails verification, `open_precomputed()` returns `None`.
 */
pub fn open_precomputed(c: &[u8],
                        &Nonce(n): &Nonce,
                        &PrecomputedKey(k): &PrecomputedKey) -> Option<Vec<u8>> {
    if c.len() < BOXZEROBYTES {
        return None
    }
    let (m, ret) = marshal(c, BOXZEROBYTES, ZEROBYTES, |dst, src, len| {
        unsafe {
            crypto_box_curve25519xsalsa20poly1305_open_afternm(dst,
                                                               src,
                                                               len,
                                                               n.as_ptr(),
                                                               k.as_ptr())
        }
    });
    if ret == 0 {
        Some(m)
    } else {
        None
    }
}

#[test]
fn test_seal_open() {
    use randombytes::randombytes;
    for i in range(0, 256u) {
        let (pk1, sk1) = gen_keypair();
        let (pk2, sk2) = gen_keypair();
        let m = randombytes(i);
        let n = gen_nonce();
        let c = seal(m.as_slice(), &n, &pk1, &sk2);
        let opened = open(c.as_slice(), &n, &pk2, &sk1);
        assert!(Some(m) == opened);
    }
}

#[test]
fn test_seal_open_precomputed() {
    use randombytes::randombytes;
    for i in range(0, 256u) {
        let (pk1, sk1) = gen_keypair();
        let (pk2, sk2) = gen_keypair();
        let k1 = precompute(&pk1, &sk2);
        let PrecomputedKey(k1buf) = k1;
        let k2 = precompute(&pk2, &sk1);
        let PrecomputedKey(k2buf) = k2;
        assert!(k1buf == k2buf);
        let m = randombytes(i);
        let n = gen_nonce();
        let c = seal_precomputed(m.as_slice(), &n, &k1);
        let opened = open_precomputed(c.as_slice(), &n, &k2);
        assert!(Some(m) == opened);
    }
}

#[test]
fn test_seal_open_tamper() {
    use randombytes::randombytes;
    for i in range(0, 32u) {
        let (pk1, sk1) = gen_keypair();
        let (pk2, sk2) = gen_keypair();
        let m = randombytes(i);
        let n = gen_nonce();
        let mut cv = seal(m.as_slice(), &n, &pk1, &sk2);
        let c = cv.as_mut_slice();
        for j in range(0, c.len()) {
            c[j] ^= 0x20;
            assert!(None == open(c, &n, &pk2, &sk1));
            c[j] ^= 0x20;
        }
    }
}

#[test]
fn test_seal_open_precomputed_tamper() {
    use randombytes::randombytes;
    for i in range(0, 32u) {
        let (pk1, sk1) = gen_keypair();
        let (pk2, sk2) = gen_keypair();
        let k1 = precompute(&pk1, &sk2);
        let k2 = precompute(&pk2, &sk1);
        let m = randombytes(i);
        let n = gen_nonce();
        let mut cv = seal_precomputed(m.as_slice(), &n, &k1);
        let c = cv.as_mut_slice();
        for j in range(0, c.len()) {
            c[j] ^= 0x20;
            assert!(None == open_precomputed(c, &n, &k2));
            c[j] ^= 0x20;
        }
    }
}

#[test]
fn test_vector_1() {
    // corresponding to tests/box.c and tests/box3.cpp from NaCl
    let alicesk = SecretKey([0x77,0x07,0x6d,0x0a,0x73,0x18,0xa5,0x7d,
                             0x3c,0x16,0xc1,0x72,0x51,0xb2,0x66,0x45,
                             0xdf,0x4c,0x2f,0x87,0xeb,0xc0,0x99,0x2a,
                             0xb1,0x77,0xfb,0xa5,0x1d,0xb9,0x2c,0x2a]);
    let bobpk   = PublicKey([0xde,0x9e,0xdb,0x7d,0x7b,0x7d,0xc1,0xb4,
                             0xd3,0x5b,0x61,0xc2,0xec,0xe4,0x35,0x37,
                             0x3f,0x83,0x43,0xc8,0x5b,0x78,0x67,0x4d,
                             0xad,0xfc,0x7e,0x14,0x6f,0x88,0x2b,0x4f]);
    let nonce   = Nonce([0x69,0x69,0x6e,0xe9,0x55,0xb6,0x2b,0x73,
                         0xcd,0x62,0xbd,0xa8,0x75,0xfc,0x73,0xd6,
                         0x82,0x19,0xe0,0x03,0x6b,0x7a,0x0b,0x37]);
    let m = [0xbe,0x07,0x5f,0xc5,0x3c,0x81,0xf2,0xd5,
             0xcf,0x14,0x13,0x16,0xeb,0xeb,0x0c,0x7b,
             0x52,0x28,0xc5,0x2a,0x4c,0x62,0xcb,0xd4,
             0x4b,0x66,0x84,0x9b,0x64,0x24,0x4f,0xfc,
             0xe5,0xec,0xba,0xaf,0x33,0xbd,0x75,0x1a,
             0x1a,0xc7,0x28,0xd4,0x5e,0x6c,0x61,0x29,
             0x6c,0xdc,0x3c,0x01,0x23,0x35,0x61,0xf4,
             0x1d,0xb6,0x6c,0xce,0x31,0x4a,0xdb,0x31,
             0x0e,0x3b,0xe8,0x25,0x0c,0x46,0xf0,0x6d,
             0xce,0xea,0x3a,0x7f,0xa1,0x34,0x80,0x57,
             0xe2,0xf6,0x55,0x6a,0xd6,0xb1,0x31,0x8a,
             0x02,0x4a,0x83,0x8f,0x21,0xaf,0x1f,0xde,
             0x04,0x89,0x77,0xeb,0x48,0xf5,0x9f,0xfd,
             0x49,0x24,0xca,0x1c,0x60,0x90,0x2e,0x52,
             0xf0,0xa0,0x89,0xbc,0x76,0x89,0x70,0x40,
             0xe0,0x82,0xf9,0x37,0x76,0x38,0x48,0x64,
             0x5e,0x07,0x05];
    let c = seal(m, &nonce, &bobpk, &alicesk);
    let pk = precompute(&bobpk, &alicesk);
    let cpre = seal_precomputed(m, &nonce, &pk);
    let cexp = vec![0xf3,0xff,0xc7,0x70,0x3f,0x94,0x00,0xe5,
                 0x2a,0x7d,0xfb,0x4b,0x3d,0x33,0x05,0xd9,
                 0x8e,0x99,0x3b,0x9f,0x48,0x68,0x12,0x73,
                 0xc2,0x96,0x50,0xba,0x32,0xfc,0x76,0xce,
                 0x48,0x33,0x2e,0xa7,0x16,0x4d,0x96,0xa4,
                 0x47,0x6f,0xb8,0xc5,0x31,0xa1,0x18,0x6a,
                 0xc0,0xdf,0xc1,0x7c,0x98,0xdc,0xe8,0x7b,
                 0x4d,0xa7,0xf0,0x11,0xec,0x48,0xc9,0x72,
                 0x71,0xd2,0xc2,0x0f,0x9b,0x92,0x8f,0xe2,
                 0x27,0x0d,0x6f,0xb8,0x63,0xd5,0x17,0x38,
                 0xb4,0x8e,0xee,0xe3,0x14,0xa7,0xcc,0x8a,
                 0xb9,0x32,0x16,0x45,0x48,0xe5,0x26,0xae,
                 0x90,0x22,0x43,0x68,0x51,0x7a,0xcf,0xea,
                 0xbd,0x6b,0xb3,0x73,0x2b,0xc0,0xe9,0xda,
                 0x99,0x83,0x2b,0x61,0xca,0x01,0xb6,0xde,
                 0x56,0x24,0x4a,0x9e,0x88,0xd5,0xf9,0xb3,
                 0x79,0x73,0xf6,0x22,0xa4,0x3d,0x14,0xa6,
                 0x59,0x9b,0x1f,0x65,0x4c,0xb4,0x5a,0x74,
                 0xe3,0x55,0xa5];
    assert!(c == cexp);
    assert!(cpre == cexp);
}

#[test]
fn test_vector_2() {
    // corresponding to tests/box2.c and tests/box4.cpp from NaCl
    let bobsk = SecretKey([0x5d,0xab,0x08,0x7e,0x62,0x4a,0x8a,0x4b,
                           0x79,0xe1,0x7f,0x8b,0x83,0x80,0x0e,0xe6,
                           0x6f,0x3b,0xb1,0x29,0x26,0x18,0xb6,0xfd,
                           0x1c,0x2f,0x8b,0x27,0xff,0x88,0xe0,0xeb]);
    let alicepk = PublicKey([0x85,0x20,0xf0,0x09,0x89,0x30,0xa7,0x54,
                             0x74,0x8b,0x7d,0xdc,0xb4,0x3e,0xf7,0x5a,
                             0x0d,0xbf,0x3a,0x0d,0x26,0x38,0x1a,0xf4,
                             0xeb,0xa4,0xa9,0x8e,0xaa,0x9b,0x4e,0x6a]);
    let nonce = Nonce([0x69,0x69,0x6e,0xe9,0x55,0xb6,0x2b,0x73,
                       0xcd,0x62,0xbd,0xa8,0x75,0xfc,0x73,0xd6,
                       0x82,0x19,0xe0,0x03,0x6b,0x7a,0x0b,0x37]);
    let c = [0xf3,0xff,0xc7,0x70,0x3f,0x94,0x00,0xe5,
             0x2a,0x7d,0xfb,0x4b,0x3d,0x33,0x05,0xd9,
             0x8e,0x99,0x3b,0x9f,0x48,0x68,0x12,0x73,
             0xc2,0x96,0x50,0xba,0x32,0xfc,0x76,0xce,
             0x48,0x33,0x2e,0xa7,0x16,0x4d,0x96,0xa4,
             0x47,0x6f,0xb8,0xc5,0x31,0xa1,0x18,0x6a,
             0xc0,0xdf,0xc1,0x7c,0x98,0xdc,0xe8,0x7b,
             0x4d,0xa7,0xf0,0x11,0xec,0x48,0xc9,0x72,
             0x71,0xd2,0xc2,0x0f,0x9b,0x92,0x8f,0xe2,
             0x27,0x0d,0x6f,0xb8,0x63,0xd5,0x17,0x38,
             0xb4,0x8e,0xee,0xe3,0x14,0xa7,0xcc,0x8a,
             0xb9,0x32,0x16,0x45,0x48,0xe5,0x26,0xae,
             0x90,0x22,0x43,0x68,0x51,0x7a,0xcf,0xea,
             0xbd,0x6b,0xb3,0x73,0x2b,0xc0,0xe9,0xda,
             0x99,0x83,0x2b,0x61,0xca,0x01,0xb6,0xde,
             0x56,0x24,0x4a,0x9e,0x88,0xd5,0xf9,0xb3,
             0x79,0x73,0xf6,0x22,0xa4,0x3d,0x14,0xa6,
             0x59,0x9b,0x1f,0x65,0x4c,0xb4,0x5a,0x74,
             0xe3,0x55,0xa5];
    let mexp = Some(vec![0xbe,0x07,0x5f,0xc5,0x3c,0x81,0xf2,0xd5,
                      0xcf,0x14,0x13,0x16,0xeb,0xeb,0x0c,0x7b,
                      0x52,0x28,0xc5,0x2a,0x4c,0x62,0xcb,0xd4,
                      0x4b,0x66,0x84,0x9b,0x64,0x24,0x4f,0xfc,
                      0xe5,0xec,0xba,0xaf,0x33,0xbd,0x75,0x1a,
                      0x1a,0xc7,0x28,0xd4,0x5e,0x6c,0x61,0x29,
                      0x6c,0xdc,0x3c,0x01,0x23,0x35,0x61,0xf4,
                      0x1d,0xb6,0x6c,0xce,0x31,0x4a,0xdb,0x31,
                      0x0e,0x3b,0xe8,0x25,0x0c,0x46,0xf0,0x6d,
                      0xce,0xea,0x3a,0x7f,0xa1,0x34,0x80,0x57,
                      0xe2,0xf6,0x55,0x6a,0xd6,0xb1,0x31,0x8a,
                      0x02,0x4a,0x83,0x8f,0x21,0xaf,0x1f,0xde,
                      0x04,0x89,0x77,0xeb,0x48,0xf5,0x9f,0xfd,
                      0x49,0x24,0xca,0x1c,0x60,0x90,0x2e,0x52,
                      0xf0,0xa0,0x89,0xbc,0x76,0x89,0x70,0x40,
                      0xe0,0x82,0xf9,0x37,0x76,0x38,0x48,0x64,
                      0x5e,0x07,0x05]);
    let m = open(c, &nonce, &alicepk, &bobsk);
    let pk = precompute(&alicepk, &bobsk);
    let m_pre = open_precomputed(c, &nonce, &pk);
    assert!(m == mexp);
    assert!(m_pre == mexp);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::rand::Rng;
    use std::rand::task_rng;
    use std::fmt;

    impl fmt::Show for SecretKey {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.as_slice().fmt(f)
        }
    }

    fn random_24_bytes<R:Rng>(rng: &mut R) -> [u8, ..24] {
        let mut ret = [0u8, ..24];
        rng.fill_bytes(ret);
        ret
    }

    fn random_32_bytes<R:Rng>(rng: &mut R) -> [u8, ..32] {
        let mut ret = [0u8, ..32];
        rng.fill_bytes(ret);
        ret
    }

    #[test]
    fn nonce_slice() {
        let mut rng = task_rng();
        let nonce_raw = random_24_bytes(&mut rng);
        let nonce = Nonce::from_slice(nonce_raw).unwrap();
        let nonce_by_ref = Nonce::from_slice_by_ref(nonce_raw).unwrap();

        assert_eq!(nonce_raw.as_slice(), nonce.as_slice());
        assert_eq!(nonce_raw.as_slice(), nonce_by_ref.as_slice());
        assert_eq!(nonce, *nonce_by_ref);
        assert_eq!(nonce.as_slice(), nonce_by_ref.as_slice());
    }

    #[test]
    fn publickey_slice() {
        let mut rng = task_rng();
        let publickey_raw = random_32_bytes(&mut rng);
        let publickey = PublicKey::from_slice(publickey_raw).unwrap();
        let publickey_by_ref = PublicKey::from_slice_by_ref(publickey_raw).unwrap();

        assert_eq!(publickey_raw.as_slice(), publickey.as_slice());
        assert_eq!(publickey_raw.as_slice(), publickey_by_ref.as_slice());
        assert_eq!(publickey, *publickey_by_ref);
        assert_eq!(publickey.as_slice(), publickey_by_ref.as_slice());
    }

    #[test]
    fn secretkey_slice() {
        let mut rng = task_rng();
        let secretkey_raw = random_32_bytes(&mut rng);
        let secretkey = PublicKey::from_slice(secretkey_raw).unwrap();
        let secretkey_by_ref = PublicKey::from_slice_by_ref(secretkey_raw).unwrap();

        assert_eq!(secretkey_raw.as_slice(), secretkey.as_slice());
        assert_eq!(secretkey_raw.as_slice(), secretkey_by_ref.as_slice());
        assert_eq!(secretkey, *secretkey_by_ref);
        assert_eq!(secretkey.as_slice(), secretkey_by_ref.as_slice());
    }
}
