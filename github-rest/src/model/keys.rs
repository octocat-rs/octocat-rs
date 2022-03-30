use crate::model::prelude::*;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Key {
    pub id: usize,
    pub key: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct GpgKey {
    #[serde(flatten)]
    pub key_info: KeyInfo,
    pub emails: Vec<Email>,
    pub subkeys: Vec<SubKey>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct SubKey {
    #[serde(flatten)]
    pub key_info: KeyInfo,
    pub emails: Vec<Email>,
    // TODO: Verify that this is indeed the correct type.
    pub subkeys: Vec<SubKey>,
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
    pub public_key: String,
    pub can_sign: bool,
    pub can_encrypt_comms: bool,
    pub can_encrypt_storage: bool,
    pub can_certify: bool,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub raw_key: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Email {
    pub email: String,
    pub verified: bool,
}
