use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce };
use rand::{distr::Alphanumeric, rng, Rng};
use base64::{Engine as _, engine::general_purpose::STANDARD};


pub mod key_loader;

pub struct PasswordManager { 
    key_data: key_loader::KeyLoader,
}

impl PasswordManager { 
    pub fn new() -> Result<PasswordManager, &'static str> { 
        let __key_data = match key_loader::KeyLoader::new() { 
            Ok(value) => value, 
            Err(err) => {
                return Err(err);
            }
        };

        Ok(PasswordManager { key_data: __key_data })
    }

    pub fn generate_password(&self) -> String { 
        let mut __new_password = "".to_string();

         let __pass_data = rng()
            .sample_iter(&Alphanumeric)  
            .take(self.key_data.get_password_length())  
            .map(char::from);


        for (idx, char) in __pass_data.enumerate() { 
            __new_password.insert(idx, char);
        }

        __new_password
    }

    pub fn encrypt_password(&self, password_string: String) -> String { 
        let __key = Key::<Aes256Gcm>::from_slice(&self.key_data.get_secret_key()).clone();
        let __cipher = Aes256Gcm::new(&__key);
        let __nonce = Nonce::from_slice(b"UniQue-noNce"); // Must be 12 bytes
        let __ciphertext = __cipher.encrypt(__nonce, password_string.as_bytes()).expect("Encryption failed");


        STANDARD.encode(__ciphertext)
    }
}