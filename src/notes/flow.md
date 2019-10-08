## General
* Outbound headers : `X-Timestamp`
* email `base32_encode(sha1(email)))` (sha1!? :horrified:)
* metrics for email `base32_encode(sha256(localpart))@host.dom`

## Secrets
* generate secrets
    * Secrets are read from a file and are `{site},{digits/ttl?}:{base32 string?}`

## BrowserID / supportdoc
* `hostname` -> support doc path
    * caches value for run? (FIFO? )
    * fetches public key from server (from support doc)
    * trusted_issuer? 
        * host == issuer
        * issuer in trusted secondaries
        * hostname delegates to issuer
    

### Crypto -> Python | M2Crypto based
* to/from PEM
* RSA key - verify/sign
* DSA key - from PEM verify/sign
* integers to/from MPInts

### Verifiers
* assertion? 
    * Provided as JWT
    * (certificate ~ certificate... ~ assertion) *-or-* JSON `{"certificates":[...], "assertion": "..."}`
* verify (not implemented)
* check_audience 
    * verify assertion

## OAuth Verifier
* https://docs.rs/oauth2/3.0.0-alpha.3/oauth2/ ?
* Uses custom FxA client library
    * will need to replicate functions
