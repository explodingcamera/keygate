# How should auth work?

you sign in, get a refresh token stored in a cookie
you can then exchange that token for a JWT, the access token
if the JWT is expired, you can exchange the refresh token for a new refresh and access token
the access tokens have an id that is stored in the refresh token table and can be used to revoke the token
if reuse is detected, all tokens associated with that session are revoked and the access token ids are added to the blacklist

- **browsers**: the refresh token is stored in a cookie, the access token is stored in memory
- **native**: the refresh are both stored in secure storage (keychain, keystore, etc), the access token is stored in memory

## JWT Support

- only support EdDSA (Ed25519) for now
- https://auth0.com/docs/secure/tokens/json-web-tokens/json-web-key-sets

https://humanstxt.org
https://securitytxt.org
https://github.com/w3c/webappsec-change-password-url
