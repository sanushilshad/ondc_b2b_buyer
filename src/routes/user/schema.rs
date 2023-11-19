use secrecy::Secret;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// macro_rules! impl_serialize_format {
//     ($struct_name:ident, $trait_name:path) => {
//         impl $trait_name for $struct_name {
//             fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//                 fmt_json(self, f)
//             }
//         }
//     };
// }

// #[derive(Debug)]
// struct SecretString(Secret<String>);

// impl_serialize_format!(AuthenticateRequest, Debug);
// #[strum(serialize_all = "snake_case")]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum AuthenticationScope {
    OTP,
    Password,
}

#[derive(Deserialize, Debug)]
pub struct AuthenticateRequest {
    _scope: AuthenticationScope,
    _identifier: String,
    // #[serde(with = "SecretString")]
    _secret: Secret<String>,
}

// impl Serialize for SecretString {
//     fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
//         serializer.serialize_str(&self.0.expose_secret())
//     }
// }