# Some notes about OAuth Verification

For OAuth, we need to have a key. This is normally specified in a configuration file, and the python tokenserver specifies this as
```ini
[tokenserver]
jwks = {"keys": [{"use": "sig", "e": "AQAB", "kty": "RSA", "alg": "RS256", "fxa-createdAt": 1564502400, "n": "15OpVGC7ws_SlU0gRbRh1Iwo8_gR8ElX2CDnbN5blKyXLg-ll0ogktoDXc-tDvTabRTxi7AXU0wWQ247odhHT47y5uz0GASYXdfPponynQ_xR9CpNn1eEL1gvDhQN9rfPIzfncl8FUi9V4WMd5f600QC81yDw9dX-Z8gdkru0aDaoEKF9-wU2TqrCNcQdiJCX9BISotjz_9cmGwKXFEekQNJWBeRQxH2bUmgwUK0HaqwW9WbYOs-zstNXXWFsgK9fbDQqQeGehXLZM4Cy5Mgl_iuSvnT3rLzPo2BmlxMLUvRqBx3_v8BTtwmNGA0v9O0FJS_mnDq0Iue0Dz8BssQCQ", "kid": "20190730-15e473fd"}]}
```

The `config` crate looks like it has a bug so I don't know how (or if) we can read in the original tokenserver.ini file for a lot of these. I strongly suspect we can't, but if you want to take a shot at seeing what `config.merge()` returns if you hand it an .ini file that has subsections like what the python tokenserver uses, have at it.

Getting back to the python tokenserver code, all of the authorizations are handled inside of the [`PyFxA` library](https://github.com/mozilla/PyFxA/), specifically, the [`oauth.py`](https://github.com/mozilla/PyFxA/blob/main/fxa/oauth.py) file.

I know we talked about using the `oauth` crate, but I think that might be more for clients wanting to use oauth to authenticate rather than servers using `oauth` to verify. Token server is more about verifying tokens, so I'm not sure how useful that crate might be. I think it might be useful to look at [the `jwt` crate](https://crates.io/crates/jwt) instead, since it provides a bunch of claim parsing and signing functions.

There's a lot of stuff in that file, but the [`verify_token`](https://github.com/mozilla/PyFxA/blob/main/fxa/oauth.py#L257) function is the more interesting part. You can see that it takes a `token` and an optional `scope`. It checks and decodes the JWT object (skipping `aud` verification), checks the JWT header, and returns the broken out token info. The only time it calls out to the remote `/verify` server is if the passed token doesn't decode properly for some reason. We can probably return an error and add a `// TODO:` at that point.

Once we have that, we can look back [into the `verifiers.py`](https://github.com/mozilla-services/tokenserver/blob/master/tokenserver/verifiers.py#L228-L245) to figure out what to do next, which is mostly composing the return result and doing a bit of extra checking on the info being returned from the JWT. 
