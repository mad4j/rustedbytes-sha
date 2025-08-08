use clap::ValueEnum;
use sha1::{Digest as Sha1Digest, Sha1};
use sha2::{Sha224, Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use blake2::{Blake2b512, Blake2s256};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum HashAlgorithm {
    /// SHA-1 (160-bit) - Legacy, not recommended for security
    #[value(name = "sha1")]
    Sha1,
    /// SHA-224 (224-bit)
    #[value(name = "sha224")]
    Sha224,
    /// SHA-256 (256-bit) - Most common
    #[value(name = "sha256")]
    Sha256,
    /// SHA-384 (384-bit)
    #[value(name = "sha384")]
    Sha384,
    /// SHA-512 (512-bit)
    #[value(name = "sha512")]
    Sha512,
    /// SHA3-224 (224-bit)
    #[value(name = "sha3-224")]
    Sha3_224,
    /// SHA3-256 (256-bit)
    #[value(name = "sha3-256")]
    Sha3_256,
    /// SHA3-384 (384-bit)
    #[value(name = "sha3-384")]
    Sha3_384,
    /// SHA3-512 (512-bit)
    #[value(name = "sha3-512")]
    Sha3_512,
    /// BLAKE2b-512 (512-bit) - High performance
    #[value(name = "blake2b")]
    Blake2b,
    /// BLAKE2s-256 (256-bit) - High performance, smaller output
    #[value(name = "blake2s")]
    Blake2s,
}

impl HashAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            HashAlgorithm::Sha1 => "SHA-1",
            HashAlgorithm::Sha224 => "SHA-224",
            HashAlgorithm::Sha256 => "SHA-256",
            HashAlgorithm::Sha384 => "SHA-384",
            HashAlgorithm::Sha512 => "SHA-512",
            HashAlgorithm::Sha3_224 => "SHA3-224",
            HashAlgorithm::Sha3_256 => "SHA3-256",
            HashAlgorithm::Sha3_384 => "SHA3-384",
            HashAlgorithm::Sha3_512 => "SHA3-512",
            HashAlgorithm::Blake2b => "BLAKE2b-512",
            HashAlgorithm::Blake2s => "BLAKE2s-256",
        }
    }
}

pub fn calculate_hash(data: &[u8], algorithm: HashAlgorithm, is_empty: bool) -> String {
    if is_empty {
        return calculate_empty_hash(algorithm);
    }
    
    let hash_bytes = match algorithm {
        HashAlgorithm::Sha1 => {
            let mut hasher = Sha1::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        },
        HashAlgorithm::Sha224 => {
            let mut hasher = Sha224::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        },
        HashAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        },
        HashAlgorithm::Sha384 => {
            let mut hasher = Sha384::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        },
        HashAlgorithm::Sha512 => {
            let mut hasher = Sha512::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        },
        HashAlgorithm::Sha3_224 => {
            let mut hasher = Sha3_224::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        },
        HashAlgorithm::Sha3_256 => {
            let mut hasher = Sha3_256::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        },
        HashAlgorithm::Sha3_384 => {
            let mut hasher = Sha3_384::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        },
        HashAlgorithm::Sha3_512 => {
            let mut hasher = Sha3_512::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        },
        HashAlgorithm::Blake2b => {
            let mut hasher = Blake2b512::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        },
        HashAlgorithm::Blake2s => {
            let mut hasher = Blake2s256::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        },
    };
    
    hex::encode(hash_bytes)
}

fn calculate_empty_hash(algorithm: HashAlgorithm) -> String {
    let hash_bytes = match algorithm {
        HashAlgorithm::Sha1 => Sha1::new().finalize().to_vec(),
        HashAlgorithm::Sha224 => Sha224::new().finalize().to_vec(),
        HashAlgorithm::Sha256 => Sha256::new().finalize().to_vec(),
        HashAlgorithm::Sha384 => Sha384::new().finalize().to_vec(),
        HashAlgorithm::Sha512 => Sha512::new().finalize().to_vec(),
        HashAlgorithm::Sha3_224 => Sha3_224::new().finalize().to_vec(),
        HashAlgorithm::Sha3_256 => Sha3_256::new().finalize().to_vec(),
        HashAlgorithm::Sha3_384 => Sha3_384::new().finalize().to_vec(),
        HashAlgorithm::Sha3_512 => Sha3_512::new().finalize().to_vec(),
        HashAlgorithm::Blake2b => Blake2b512::new().finalize().to_vec(),
        HashAlgorithm::Blake2s => Blake2s256::new().finalize().to_vec(),
    };
    
    hex::encode(hash_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_known_vectors() {
        // Test vector from NIST
        let test_cases = vec![
            ("", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            ("abc", "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"),
            ("message digest", "f7846f55cf23e14eebeab5b4e1550cad5b509e3348fbc4efa3a1413d393cb650"),
        ];

        for (input, expected) in test_cases {
            let result = if input.is_empty() {
                calculate_hash(&[], HashAlgorithm::Sha256, true)
            } else {
                calculate_hash(input.as_bytes(), HashAlgorithm::Sha256, false)
            };
            assert_eq!(result, expected, "SHA-256 failed for input: '{}'", input);
        }
    }

    #[test]
    fn test_sha1_known_vectors() {
        let test_cases = vec![
            ("", "da39a3ee5e6b4b0d3255bfef95601890afd80709"),
            ("abc", "a9993e364706816aba3e25717850c26c9cd0d89d"),
        ];

        for (input, expected) in test_cases {
            let result = if input.is_empty() {
                calculate_hash(&[], HashAlgorithm::Sha1, true)
            } else {
                calculate_hash(input.as_bytes(), HashAlgorithm::Sha1, false)
            };
            assert_eq!(result, expected, "SHA-1 failed for input: '{}'", input);
        }
    }

    #[test]
    fn test_sha512_known_vectors() {
        let test_cases = vec![
            ("abc", "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"),
        ];

        for (input, expected) in test_cases {
            let result = calculate_hash(input.as_bytes(), HashAlgorithm::Sha512, false);
            assert_eq!(result, expected, "SHA-512 failed for input: '{}'", input);
        }
    }

    #[test]
    fn test_different_algorithms_produce_different_hashes() {
        let input = b"test data";
        
        let sha1 = calculate_hash(input, HashAlgorithm::Sha1, false);
        let sha256 = calculate_hash(input, HashAlgorithm::Sha256, false);
        let sha512 = calculate_hash(input, HashAlgorithm::Sha512, false);
        let blake2b = calculate_hash(input, HashAlgorithm::Blake2b, false);
        
        // All hashes should be different
        assert_ne!(sha1, sha256);
        assert_ne!(sha256, sha512);
        assert_ne!(sha512, blake2b);
        assert_ne!(sha1, blake2b);
        
        // Check expected lengths (in hex characters)
        assert_eq!(sha1.len(), 40);    // 160 bits = 40 hex chars
        assert_eq!(sha256.len(), 64);  // 256 bits = 64 hex chars
        assert_eq!(sha512.len(), 128); // 512 bits = 128 hex chars
        assert_eq!(blake2b.len(), 128); // 512 bits = 128 hex chars
    }

    #[test]
    fn test_hash_algorithm_names() {
        assert_eq!(HashAlgorithm::Sha256.name(), "SHA-256");
        assert_eq!(HashAlgorithm::Sha1.name(), "SHA-1");
        assert_eq!(HashAlgorithm::Blake2b.name(), "BLAKE2b-512");
    }
}