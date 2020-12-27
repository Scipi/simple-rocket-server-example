use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sha3::{Digest, Sha3_512};

/// Returns a base64-encoded SHA3-512 hash of the salt+password inputs
///
/// # Arguments
///
/// * `salt` - The salt portion of the hash input
/// * `password` - The cleartext password to hash
///
/// # Examples
///
/// ```
/// use auth::login_auth;
/// let pw_hash = login_auth::hash("salt", "password");
/// ```
pub fn hash(salt: &str, password: &str) -> String {
    let mut hasher = Sha3_512::new();

    hasher.update(salt.as_bytes());
    hasher.update(password.as_bytes());

    let result = hasher.finalize();
    let result = result.as_slice();

    base64::encode(result)
}

/// Returns a random salt of `len` alphanumeric characters
///
/// * Arguments
///
/// `len` - Length of salt to generate
///
/// * Example
///
/// ```
/// use login_auth;
///
/// let salt = login_auth::generate_salt(256);
/// ```
pub fn generate_salt(len: usize) -> String {
    let mut rng = thread_rng();
    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(len)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    #[test]
    fn test_hash() {
        let expected = hex!(
        "a4d53131134530f701f930e59af6d301fa350b06b762a3850535b13400685a3aea6fe190481a882c9540b1b8c00bf45044312fc125588dff349ce47b1cd3bccd"
        );

        let expected = base64::encode(expected);

        assert_eq!(hash("salt", "asdf1234"), expected);
    }
}