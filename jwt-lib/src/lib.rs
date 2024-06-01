use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
  pub email: String,
  password: String,
}

//https://crates.io/crates/jsonwebtoken
//Copy fields from User struct because we need to deserialize it
#[derive(Serialize, Deserialize)]
struct Claims {
  email: String,
  password: String,
  exp: i64,
}

//jsonwebtoken encode: https://docs.rs/jsonwebtoken/latest/jsonwebtoken/fn.encode.html
// https://crates.io/crates/jsonwebtoken
// HS256, HS384, or HS512: same key to encode and decode; RSA: private key to encode, and public key to decode
pub fn get_jwt(user: User) -> Result<String, String> {
  //TODO: check with db if the user password is correct
  
  // generate jwt from jsonwebtoken crate
  // https://crates.io/crates/jsonwebtoken
  let token = encode(
    &Header::default(),
    &Claims {
      email: user.email,
      password: user.password,
      exp: (Utc::now() + Duration::minutes(1)).timestamp(),//expires after 1 min
    },
    &EncodingKey::from_secret("mykey".as_bytes()),
  )
  .map_err(|e| e.to_string());

  return token;
}

// jsonwebtoken Validation struct: https://docs.rs/jsonwebtoken/latest/jsonwebtoken/struct.Validation.html
pub fn decode_jwt(token: &str) -> Result<User, String> {
  let token_data = decode::<User>(
    token,
    &DecodingKey::from_secret("mykey".as_bytes()),
    &Validation::default(),
  );//default leeway is 60 secs, can be changed at validation options: https://docs.rs/jsonwebtoken/9.3.0/jsonwebtoken/struct.Validation.html

  match token_data {
    Ok(token_data) => Ok(token_data.claims),

    Err(e) => Err(e.to_string()),
  }
}
