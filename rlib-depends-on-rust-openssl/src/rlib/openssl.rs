
use std::mem;

use bytes::{BufMut, Bytes, BytesMut};
use openssl::symm;
use rand::{self, Rng};
use rlib::digest::{self, DigestType, Digest};

pub enum Error {
    OpenSSLError(::openssl::error::ErrorStack),
}

impl From<::openssl::error::ErrorStack> for Error {
    fn from(e: ::openssl::error::ErrorStack) -> Error {
        Error::OpenSSLError(e)
    }
}

fn bytes_to_key(key_size: usize, iv_size: usize, key: &[u8]) -> Bytes {
    if iv_size + key_size == 0 {
        return Bytes::new();
    }

    let mut digest = digest::with_type(DigestType::Md5);
    let total_loop = (key_size + iv_size + digest.digest_len() - 1) / digest.digest_len();
    let m_length = digest.digest_len() + key.len();
    let mut result = BytesMut::with_capacity(total_loop * digest.digest_len());
    let mut m = BytesMut::with_capacity(key.len());
    for _ in 0..total_loop {
        let mut vkey = mem::replace(&mut m, BytesMut::with_capacity(m_length));
        vkey.put(key);

        digest.update(&vkey);
        digest.digest(&mut m);
        digest.reset();

        result.put_slice(&m);
    }

    result.truncate(key_size);
    result.freeze()
}

fn gen_random_bytes(len: usize) -> Bytes {
    let mut bytes = BytesMut::with_capacity(len);
    unsafe {
        bytes.set_len(len);
    }
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes.freeze()
}

/// Core cipher of OpenSSL
pub struct OpenSSLCrypto {
    cipher: symm::Cipher,
    inner: symm::Crypter,
}

impl OpenSSLCrypto {

    /// Creates by type
    pub fn new() -> OpenSSLCrypto { //key: &[u8], iv: &[u8]
        let key_size: usize = symm::Cipher::aes_128_cfb128().key_len();
        let iv_size: usize = symm::Cipher::aes_128_cfb128().iv_len().unwrap_or(0);
        let key_bytes: Bytes = bytes_to_key(key_size, iv_size, b"my_password");
        let iv_bytes: Bytes = gen_random_bytes(iv_size);
        let key: &[u8] = &key_bytes[0..];
        let iv: &[u8] = &iv_bytes[0..];

        let cipher = symm::Cipher::aes_128_cfb128();
        let inner = symm::Crypter::new(cipher, symm::Mode::Encrypt, key, Some(iv)).unwrap();

        OpenSSLCrypto {
            cipher: cipher,
            inner: inner,
        }
    }

    /// Update data
    pub fn update<B: BufMut>(&mut self, data: &[u8], out: &mut B) -> Result<(), Error> {
        let least_reserved = data.len() + self.cipher.block_size();
        let mut buf = BytesMut::with_capacity(least_reserved); // NOTE: len() is 0 now!
        unsafe {
            buf.set_len(least_reserved);
        }
        let length = self.inner.update(data, &mut *buf)?;
        buf.truncate(length);
        out.put(buf);
        Ok(())
    }

    /// Generate the final block
    pub fn finalize<B: BufMut>(&mut self, out: &mut B) -> Result<(), Error> {
        let least_reserved = self.cipher.block_size();
        let mut buf = BytesMut::with_capacity(least_reserved); // NOTE: len() is 0 now!
        unsafe {
            buf.set_len(least_reserved);
        }

        let length = self.inner.finalize(&mut *buf)?;
        buf.truncate(length);
        out.put(buf);
        Ok(())
    }

    /// Gets output buffer size based on data
    pub fn buffer_size(&self, data: &[u8]) -> usize {
        self.cipher.block_size() + data.len()
    }
}
