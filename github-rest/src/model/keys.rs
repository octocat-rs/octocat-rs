use crate::model::{keys::nested::*, prelude::*};

/// <https://docs.github.com/en/rest/users/keys#list-public-keys-for-a-user=>
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct SshKey {
    pub id: usize,
    pub key: String,
}

/// <https://docs.github.com/en/rest/users/gpg-keys#list-gpg-keys-for-a-user=>
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct GpgKey {
    pub emails: Vec<Email>,
    pub subkeys: Vec<SubKey>,
    pub revoked: bool,
    #[serde(flatten)]
    pub key_info: KeyInfo,
}

pub mod nested {
    use crate::model::prelude::*;

    #[derive(Serialize, Deserialize, Default, Clone, Debug)]
    pub struct SubKey {
        pub emails: Vec<Email>,
        pub subkeys: Vec<SubKey>,
        #[serde(flatten)]
        pub key_info: KeyInfo,
    }

    /// Contains general info about a key, including:
    /// * The public key,
    /// * Its creation and (optional) expiration date,
    /// * Whether it can sign,
    /// * What it can (and can't) encrypt,
    /// * etc.
    #[derive(Serialize, Deserialize, Default, Clone, Debug)]
    pub struct KeyInfo {
        pub id: usize,
        pub primary_key_id: Option<usize>,
        pub key_id: String,
        pub raw_key: Option<String>,
        pub public_key: String,
        pub created_at: String,
        pub expires_at: Option<String>,
        pub can_sign: bool,
        pub can_encrypt_comms: bool,
        pub can_encrypt_storage: bool,
        pub can_certify: bool,
    }

    #[derive(Serialize, Deserialize, Default, Clone, Debug)]
    pub struct Email {
        pub email: String,
        pub verified: bool,
    }
}
