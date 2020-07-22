use crate::settings;
use crate::token::{verify_jwt_token_from_rsa, Claims};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Response {
    pub email: String,
    pub claims: Claims,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Key {
    // kty: String,
    pub n: String, // We only need n and e
    pub e: String,
    // qi: String,
    // kid: String,
    // alg: String,
    // r#use: String,
    // fxa-createdAt: String,   // This can't be named this hence serde_json
    // won't parse this resulting in error
}

#[derive(Serialize, Deserialize)]
pub struct JWK {
    pub keys: Vec<Key>,
}

pub fn verify(token: &str, jwks: &JWK) -> Result<Response, ()> {
    // let jwks: &str = r#"{"keys":[{"kty":"RSA","n":"nW_losfifTdqolJzRvQEHYLzjf25eX7MriczYrUnbr25runIyz214WAuTeAECDpXGJo__J6brUugkLFaf_NGv-JpJ44QKUiZKcw7qB1N3sEy2WF3XbUR0W0w28pfA2WbwcTRb1j0mj0KPWltCFCK51_KeINMuCTDC9UyXUZjwpSQyJ6lYQVK_n2XR8K2qohOE8I3k03dRkZmZ_D6DLHUUD7hp6pdUpvp2Q6pl_AI59s1J3Z-tCgy_N7ja9QdXE8K6hFAjoF3p5ix46vo6M6HeUGVkVrjEa-Lh15dFkmf6_-8N0r9owwNxpNqkT2nzVdZY2LwLzzqqmgzfP0lbhziaw","e":"AQAB","dp":"aod_c9v-N82vmOppJQkIUjSOf_pkmrxJZZ9eJO-ebJd5OsxN_GLOFHa3AH0-vlUoiwFOsziB9yq33EkQT0r9BYcwXEvHJKX5smt17wmIskakLw2FWozSwNf9bgCPoIBh2NyVtcJ0p1SaO3IuIuQsQetfmwkqHbdKOYUnuNc0IuE","dq":"muc3N3YzJ87RLiBij6xfAliSxdMDg6zKBFXwPRHQJJ0cg6lbvnpnp8XJjjhmYov_2xmICi3C_LO6fwe8KyUOyiPkb0VbjWZtq4Iol9qkQ0iKTnGXkoTfBHVheGq5QoAhxiX7xExd4Gnog5KocrexFWuiZQ0Ul22Bji3gqJhwvcE","qi":"xguY_G6Ld0Rp7a_ZHAFnAr3Q5Dzhjhkp3vgCi1uNp2jmP3QYng-GvP2xaLcLA0HLBOc0ghgSJYcnmmOB6bxVkVc5R0Hg17-tLlOgQejCd5mQUeMmp_upAScPHzoEea-OM9O_mHtM5BuuroaLIJdhxYolRkKfwD35cwdMX2j9H_4","kid":"20191118-e43b24c6","alg":"RS256","use":"sig","fxa-createdAt":1574056800}]}"#;

    let keys: &Vec<Key> = &jwks.keys;

    let mut my_claims: Option<Claims> = None;

    for key in keys {
        match verify_jwt_token_from_rsa(&key, token) {
            Ok(claims) => {
                my_claims = Some(claims.claims);
                break;
            }
            Err(e) => println!("{:?}", e),
        }
    }

    if my_claims == None {
        // This is the condition when we should get `/verify` but we error
        // out for now.
        return Err(());
    }

    if let Some(claims) = my_claims {
        return Ok(Response {
            email: "member@example.com".to_string(),
            claims,
        });
    } else {
        return Err(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::generate_token;
    use chrono::Utc;

    #[test]
    fn test_jwks() {
        static THREE_DAYS: i64 = 60 * 60 * 24 * 3;
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; //nanoseconds -> seconds
        let my_claims = Claims {
            user: "dummy_user".to_string(),
            scope: "None".to_string(),
            client_id: "bhj4".to_string(),
            iat: now,
            exp: now + THREE_DAYS,
            issuer: "None".to_string(),
        };

        let token = generate_token(&my_claims).unwrap();
        let jwks: &str = r#"{"keys": [{"n": "nzyis1ZjfNB0bBgKFMSvvkTtwlvBsaJq7S5wA-kzeVOVpVWwkWdVha4s38XM_pa_yr47av7-z3VTmvDRyAHcaT92whREFpLv9cj5lTeJSibyr_Mrm_YtjCZVWgaOYIhwrXwKLqPr_11inWsAkfIytvHWTxZYEcXLgAXFuUuaS3uF9gEiNQwzGTU1v0FqkqTBr4B8nW3HCN47XUu0t8Y0e-lf4s4OxQawWD79J9_5d3Ry0vbV3Am1FtGJiJvOwRsIfVChDpYStTcHTCMqtvWbV6L11BWkpzGXSW4Hv43qa-GSYOD2QU68Mb59oSk2OB-BtOLpJofmbGEGgvmwyCI9Mw", "e": "AQAB"}, {"kty":"RSA","n":"nW_losfifTdqolJzRvQEHYLzjf25eX7MriczYrUnbr25runIyz214WAuTeAECDpXGJo__J6brUugkLFaf_NGv-JpJ44QKUiZKcw7qB1N3sEy2WF3XbUR0W0w28pfA2WbwcTRb1j0mj0KPWltCFCK51_KeINMuCTDC9UyXUZjwpSQyJ6lYQVK_n2XR8K2qohOE8I3k03dRkZmZ_D6DLHUUD7hp6pdUpvp2Q6pl_AI59s1J3Z-tCgy_N7ja9QdXE8K6hFAjoF3p5ix46vo6M6HeUGVkVrjEa-Lh15dFkmf6_-8N0r9owwNxpNqkT2nzVdZY2LwLzzqqmgzfP0lbhziaw","e":"AQAB","dp":"aod_c9v-N82vmOppJQkIUjSOf_pkmrxJZZ9eJO-ebJd5OsxN_GLOFHa3AH0-vlUoiwFOsziB9yq33EkQT0r9BYcwXEvHJKX5smt17wmIskakLw2FWozSwNf9bgCPoIBh2NyVtcJ0p1SaO3IuIuQsQetfmwkqHbdKOYUnuNc0IuE","dq":"muc3N3YzJ87RLiBij6xfAliSxdMDg6zKBFXwPRHQJJ0cg6lbvnpnp8XJjjhmYov_2xmICi3C_LO6fwe8KyUOyiPkb0VbjWZtq4Iol9qkQ0iKTnGXkoTfBHVheGq5QoAhxiX7xExd4Gnog5KocrexFWuiZQ0Ul22Bji3gqJhwvcE","qi":"xguY_G6Ld0Rp7a_ZHAFnAr3Q5Dzhjhkp3vgCi1uNp2jmP3QYng-GvP2xaLcLA0HLBOc0ghgSJYcnmmOB6bxVkVc5R0Hg17-tLlOgQejCd5mQUeMmp_upAScPHzoEea-OM9O_mHtM5BuuroaLIJdhxYolRkKfwD35cwdMX2j9H_4","kid":"20191118-e43b24c6","alg":"RS256","use":"sig","fxa-createdAt":1574056800}]}"#;
        let jwks: JWK = serde_json::from_str(jwks).unwrap();

        println!("{:?}", verify(&token, &jwks)); // TODO: complete test
    }
}
