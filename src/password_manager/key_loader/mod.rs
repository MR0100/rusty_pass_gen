use std::env;

use dotenvy::dotenv;

pub struct KeyLoader {
    secret_key: String,    
    password_length: usize,
}

impl KeyLoader {
    pub fn new() -> Result<KeyLoader, &'static str> { 
        dotenv().ok();

        let __key = match env::var("SECRET_KEY") {
            Ok(value) => value, 
            Err(_) => { return Err("SECRET KEY NOT FOUND!"); }
        };

        let __len  = match env::var("PASSWORD_LENGTH") { 
            Ok(value) => value.parse::<usize>().unwrap(), 
            Err(_) => { return Err("Password Generation Length is not Specified!"); }
        };

        Ok(KeyLoader { secret_key: __key, password_length: __len, })
    }

    pub fn get_secret_key(&self) -> Vec<u8> { 
        let __key = self.secret_key.clone(); 
        __key.as_bytes().to_vec()
    }

    pub fn get_password_length(&self) -> usize { 
        self.password_length.clone()
    }


}
