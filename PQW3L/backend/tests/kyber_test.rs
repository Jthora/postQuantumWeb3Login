#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kyber_keypair_generation() {
        let (public_key, secret_key) = generate_keypair();
        assert_eq!(public_key.len(), 1568);
        assert_eq!(secret_key.len(), 3168);
    }

    #[test]
    fn test_kyber_encryption_decryption() {
        let (public_key, secret_key) = generate_keypair();
        let (ciphertext, shared_secret) = encrypt(&public_key);
        let decrypted_secret = decrypt(&secret_key, &ciphertext);
        assert_eq!(shared_secret, decrypted_secret);
    }
}