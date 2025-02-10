use wasm_bindgen::prelude::*;
use crate::pqcrypto::kyber;

/// Generate a Kyber keypair (exposed to WASM)
#[wasm_bindgen]
pub fn wasm_generate_keypair() -> JsValue {
    let (public_key, secret_key) = kyber::generate_keypair();
    JsValue::from_serde(&(public_key, secret_key)).unwrap()
}

/// Encrypt a shared secret (exposed to WASM)
#[wasm_bindgen]
pub fn wasm_encrypt(public_key: &[u8]) -> JsValue {
    let (ciphertext, shared_secret) = kyber::encrypt(public_key);
    JsValue::from_serde(&(ciphertext, shared_secret)).unwrap()
}

/// Decrypt a shared secret (exposed to WASM)
#[wasm_bindgen]
pub fn wasm_decrypt(secret_key: &[u8], ciphertext: &[u8]) -> JsValue {
    let shared_secret = kyber::decrypt(secret_key, ciphertext);
    JsValue::from_serde(&shared_secret).unwrap()
}