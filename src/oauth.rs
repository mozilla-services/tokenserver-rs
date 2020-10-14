use crate::token::{verify_jwt_token_from_rsa, Claims};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Response {
    pub email: String,
    pub claims: Claims,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Key {
    #[serde(skip)]
    kty: String,

    pub n: String, // We only need n and e
    pub e: String,

    #[serde(skip)]
    qi: String,

    #[serde(skip)]
    kid: String,

    #[serde(skip)]
    alg: String,

    #[serde(skip)]
    r#use: String,

    #[serde(skip)]
    #[serde(rename = "fxa-createdAt")]
    fxa_created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct JWK {
    pub keys: Vec<Key>,
}

fn scope_matches(provided: Vec<String>, required: Option<Vec<String>>) -> bool {
    if let Some(required_scopes) = required {
        for provided_scope in &provided {
            for required_scope in &required_scopes {
                if !match_single_scope(&provided_scope, &required_scope) {
                    return false;
                }
            }
        }
    }
    true
}

fn match_single_scope(provided: &str, required: &str) -> bool {
    let prefix = "https:".to_string();
    if provided.starts_with(&prefix) {
        _match_url_scope(provided, required)
    } else {
        match_shortname_scope(provided, required)
    }
}

fn _match_url_scope(_provided: &str, _required: &str) -> bool {
    // TODO: Implement the match_url_scope as written here
    // https://github.com/mozilla/PyFxA/blob/53a9b649dd1225a641be95ea2ab3e241533ef562/fxa/_utils.py#L124
    // More help for scope matching
    // https://github.com/mozilla/fxa-auth-server/blob/master/fxa-oauth-server/docs/scopes.md
    false
}

fn match_shortname_scope(provided: &str, required: &str) -> bool {
    let mut prov_names = provided.split(':').collect::<Vec<&str>>();
    let mut req_names = required.split(':').collect::<Vec<&str>>();
    // https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust
    let wt = &"write";
    if req_names.last() == Some(wt) {
        if prov_names.last() != Some(wt) {
            return false;
        }
        prov_names.pop();
        req_names.pop();
    } else if prov_names.last() == Some(wt) {
        prov_names.pop();
    }
    if prov_names.len() > req_names.len() {
        return false;
    }

    for p in &prov_names {
        for r in &req_names {
            if p != r {
                return false;
            }
        }
    }
    false
}

pub fn verify(token: &str, jwks: &JWK) -> Result<Response, ()> {
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
    let req_scope: Vec<String> = vec![
        "profile:write".to_string(),
        "profile:email".to_string(),
        "profile:email:write".to_string(),
    ];

    if let Some(claims) = my_claims {
        if let Some(ref scope) = claims.scope {
            if !scope_matches(scope.to_vec(), Some(req_scope)) {
                return Err(());
            }
        }

        let email = format!("{}@{}", claims.issuer, claims.user);

        return Ok(Response { email, claims });
    } else {
        // This is the condition when we should get `/verify` but we error
        // out for now.
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
        let scope_vec: Vec<String> = vec![
            "profile:write".to_string(),
            "profile:email".to_string(),
            "profile:email:write".to_string(),
        ];

        let my_claims = Claims {
            user: "dummy_user".to_string(),
            scope: Some(scope_vec),
            client_id: "bhj4".to_string(),
            iat: now,
            exp: now + THREE_DAYS,
            issuer: "None".to_string(),
        };

        let token = generate_token(&my_claims).unwrap();
        let jwks: &str = r#"{"keys": [{"n": "nzyis1ZjfNB0bBgKFMSvvkTtwlvBsaJq7S5wA-kzeVOVpVWwkWdVha4s38XM_pa_yr47av7-z3VTmvDRyAHcaT92whREFpLv9cj5lTeJSibyr_Mrm_YtjCZVWgaOYIhwrXwKLqPr_11inWsAkfIytvHWTxZYEcXLgAXFuUuaS3uF9gEiNQwzGTU1v0FqkqTBr4B8nW3HCN47XUu0t8Y0e-lf4s4OxQawWD79J9_5d3Ry0vbV3Am1FtGJiJvOwRsIfVChDpYStTcHTCMqtvWbV6L11BWkpzGXSW4Hv43qa-GSYOD2QU68Mb59oSk2OB-BtOLpJofmbGEGgvmwyCI9Mw", "e": "AQAB"}, {"kty":"RSA","n":"nW_losfifTdqolJzRvQEHYLzjf25eX7MriczYrUnbr25runIyz214WAuTeAECDpXGJo__J6brUugkLFaf_NGv-JpJ44QKUiZKcw7qB1N3sEy2WF3XbUR0W0w28pfA2WbwcTRb1j0mj0KPWltCFCK51_KeINMuCTDC9UyXUZjwpSQyJ6lYQVK_n2XR8K2qohOE8I3k03dRkZmZ_D6DLHUUD7hp6pdUpvp2Q6pl_AI59s1J3Z-tCgy_N7ja9QdXE8K6hFAjoF3p5ix46vo6M6HeUGVkVrjEa-Lh15dFkmf6_-8N0r9owwNxpNqkT2nzVdZY2LwLzzqqmgzfP0lbhziaw","e":"AQAB","dp":"aod_c9v-N82vmOppJQkIUjSOf_pkmrxJZZ9eJO-ebJd5OsxN_GLOFHa3AH0-vlUoiwFOsziB9yq33EkQT0r9BYcwXEvHJKX5smt17wmIskakLw2FWozSwNf9bgCPoIBh2NyVtcJ0p1SaO3IuIuQsQetfmwkqHbdKOYUnuNc0IuE","dq":"muc3N3YzJ87RLiBij6xfAliSxdMDg6zKBFXwPRHQJJ0cg6lbvnpnp8XJjjhmYov_2xmICi3C_LO6fwe8KyUOyiPkb0VbjWZtq4Iol9qkQ0iKTnGXkoTfBHVheGq5QoAhxiX7xExd4Gnog5KocrexFWuiZQ0Ul22Bji3gqJhwvcE","qi":"xguY_G6Ld0Rp7a_ZHAFnAr3Q5Dzhjhkp3vgCi1uNp2jmP3QYng-GvP2xaLcLA0HLBOc0ghgSJYcnmmOB6bxVkVc5R0Hg17-tLlOgQejCd5mQUeMmp_upAScPHzoEea-OM9O_mHtM5BuuroaLIJdhxYolRkKfwD35cwdMX2j9H_4","kid":"20191118-e43b24c6","alg":"RS256","use":"sig","fxa-createdAt":1574056800}]}"#;
        let jwks: JWK = serde_json::from_str(jwks).unwrap();

        assert!(verify(&token, &jwks).is_ok());
    }
}
