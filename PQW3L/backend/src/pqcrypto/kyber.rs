use pqcrypto::kem::kyber1024::*;
use pqcrypto_traits::kem::*;
use zeroize::Zeroize;

/// Generates a Kyber1024 keypair (public and secret keys)
pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
    let (public_key, secret_key) = keypair();
    (public_key.as_bytes().to_vec(), secret_key.as_bytes().to_vec())
}

/// Encrypts a shared secret using Kyber (for key exchange)
pub fn encrypt(public_key: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let public_key = PublicKey::from_bytes(public_key).expect("Invalid public key");
    let (ciphertext, shared_secret) = encapsulate(&public_key);
    (ciphertext.as_bytes().to_vec(), shared_secret.as_bytes().to_vec())
}

/// Decrypts the ciphertext to recover the shared secret
pub fn decrypt(secret_key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let secret_key = SecretKey::from_bytes(secret_key).expect("Invalid secret key");
    let ciphertext = Ciphertext::from_bytes(ciphertext).expect("Invalid ciphertext");
    let shared_secret = decapsulate(&ciphertext, &secret_key);
    shared_secret.as_bytes().to_vec()
}

/// Securely clears memory for private keys
pub fn zeroize_key(mut key: Vec<u8>) {
    key.zeroize();
}