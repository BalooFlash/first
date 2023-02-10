use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::error::ErrorUnauthorized;
use futures::future::{Ready, ok, err};

use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Algorithm, Header, EncodingKey, DecodingKey, Validation};

use chrono::Utc;
// use chrono::serde::ts_seconds;
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwToken {
    pub user_id: i32,
    pub exp: usize,
}

impl JwToken {
    // get Secret key from config.yml
    pub fn get_key() -> String {
        let config = Config::new();
        let key_str = config.map.get("SECRET_KEY")
                                .unwrap().as_str()
                                .unwrap();
        return key_str.to_owned()
    }

    // generate token 
    pub fn encode(self) -> String {
        let key = EncodingKey::from_secret(JwToken::get_key().as_ref());
        let token = encode(&Header::default(), &self, &key).unwrap();

        return token
    }

    // create JwToken
    pub fn new(user_id: i32) -> Self {
        let config = Config::new();
        let minutes = config.map.get("EXPIRE_MINUTES")
                            .unwrap().as_i64().unwrap();
        // let timestamp = Utc::now();
        let expiration = Utc::now().checked_add_signed(chrono::Duration::minutes(minutes))
                            .expect("valid timestamp")
                            .timestamp();
        return JwToken { user_id, exp: expiration as usize}
    }

    // decoding token
    pub fn from_token(token: String) -> Result<Self, String> {
        let key = DecodingKey::from_secret(
            JwToken::get_key().as_ref()
        );
        let token_result = decode::<JwToken>(&token.as_str(), &key, &Validation::new(Algorithm::HS256));
        
        match token_result {
            Ok(data) => {
                return Ok(data.claims)
            },
            Err(error) => {
                let message = format!("{}", error);
                return Err(message)
            }
        }
    }
}

impl FromRequest for JwToken {
    type Error = Error;
    type Future = Ready<Result<JwToken, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let raw_token = data.to_str().unwrap().to_string();
                let token_result = JwToken::from_token(raw_token);

                match token_result {
                    Ok(token) => {
                        return ok(token)
                    },
                    Err(message) => {
                        if message == "ExpiredSignature".to_owned() {
                            return err(
                                ErrorUnauthorized("token expired")
                            )
                        }
                        return err(
                            ErrorUnauthorized("token can't be decoded")
                        )
                    }
                }
            },
            None => {
                let error = ErrorUnauthorized(
                    "token not in header under key 'token'"
                );
                return err(error)
            }
        }
    }
}
