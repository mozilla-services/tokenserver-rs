use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn decode_token(token: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    let pub_key = include_bytes!("public_rsa_key.pem");

    decode::<Claims>(
        &token,
        &DecodingKey::from_rsa_pem(pub_key).unwrap(),
        &Validation::new(Algorithm::RS256),
    )
}

pub fn generate_token(my_claims: &Claims) -> String {
    let privkey_pem = include_bytes!("private_rsa_key.pem");

    encode(
        &Header::new(Algorithm::RS256),
        &my_claims,
        &EncodingKey::from_rsa_pem(privkey_pem).unwrap(),
    )
    .unwrap()
}

#[cfg(test)]

mod tests {

    use super::*;
    use chrono::Utc;
    use jsonwebtoken::errors::ErrorKind;

    #[test]
    fn test_token() {
        static THREE_DAYS: i64 = 60 * 60 * 24 * 3;
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; //nanoseconds -> seconds
        let my_claims = Claims {
            sub: "dummy_sub".to_string(),
            company: "dummy_company".to_string(),
            iat: now,
            exp: now + THREE_DAYS,
        };

        let token: String = generate_token(&my_claims);

        let token_data = match decode_token(&token) {
            Ok(c) => c,
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
                _ => panic!("Some other errors"),
            },
        };
    }
}
