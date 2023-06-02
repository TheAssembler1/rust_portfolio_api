// FIXME: put jwt file key init path in env var
// FIXME: put jwt claims in en var
use jwt_simple::prelude::*;
use serde::Deserialize;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::OnceLock;

pub static JWT_KEY: OnceLock<HS256Key> = OnceLock::new();

#[derive(Deserialize)]
pub struct JwtToken {
    pub token: String,
}

pub fn init_jwt_key() {
    println!("checking for jwt private key");

    let jwt_private_key_file_path = Path::new(".jwt.private.key");
    let key: HS256Key;
    if jwt_private_key_file_path.exists() {
        println!("jwt private key found");
        let mut file = File::open(jwt_private_key_file_path.to_str().unwrap()).unwrap();
        let mut buffer = Vec::new();
        let file_size = file.read_to_end(&mut buffer).unwrap();

        if file_size <= 0 {
            panic!("jwt private key file size was {}", file_size);
        }

        key = HS256Key::from_bytes(&buffer);
    } else {
        println!("jwt private key not found");
        println!("creating new jwt private key and writing to .private.key");

        key = HS256Key::generate();

        let mut file = File::create(jwt_private_key_file_path.to_str().unwrap()).unwrap();
        file.write_all(&key.to_bytes()[..]).unwrap();
    }

    JWT_KEY.set(key).unwrap();
}

pub fn jwt_create_token() -> String {
    let claims = Claims::create(Duration::from_hours(2));
    JWT_KEY.get().unwrap().authenticate(claims).unwrap()
}

impl JwtToken {
    pub fn jwt_validate_token(self) -> bool {
        let claims = JWT_KEY
            .get()
            .unwrap()
            .verify_token::<NoCustomClaims>(&self.token, None);

        claims.is_err()
    }

    pub fn new(token: String) -> Self {
        Self { token }
    }
}
