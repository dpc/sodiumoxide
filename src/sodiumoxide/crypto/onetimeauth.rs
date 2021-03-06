/*!
Secret-key One-time authentication

# Security model
The `authenticate()` function, viewed as a function
of the message for a uniform random key, is designed to meet the
standard notion of unforgeability after a single message. After the
sender authenticates one message, an attacker cannot find authenticators
for any other messages.

The sender must not use `authenticate()` to authenticate more than one message
under the same key. Authenticators for two messages under the same key should
be expected to reveal enough information to allow forgeries of authenticators
on other messages.

# Selected primitive
`authenticate()` is `crypto_onetimeauth_poly1305`, an authenticator specified
in [Cryptography in NaCl](http://nacl.cr.yp.to/valid.html), Section 9. This
authenticator is proven to meet the standard notion of unforgeability after a
single message.
*/
pub use self::poly1305::*;
#[path="auth_macros.rs"]
mod auth_macros;
#[path="poly1305.rs"]
pub mod poly1305;
