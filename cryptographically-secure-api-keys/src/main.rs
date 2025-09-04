// APIKey = Prefix + Version + base32EncodeLowercase([ UUIDv7 (16 bytes) || secret (32 bytes) ])

use base32::Alphabet;
use chrono::{DateTime, Utc};
use rand::{RngCore, rngs::ThreadRng};
use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;
use uuid::Uuid;
use zeroize::Zeroize;

const API_KEY_SECRET_SIZE: usize = 32; // 256 bits
const API_KEY_HASH_SIZE: usize = 32; // 256 bits
const API_KEY_PREFIX: &str = "elizielx";

#[derive(Debug, Clone, Serialize)]
pub struct ApiKey {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
    pub expires_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing)]
    pub version: i16,
    #[serde(skip_serializing)]
    pub secret_hash: [u8; API_KEY_HASH_SIZE],

    pub organization_id: Uuid,
}

fn hash_api_key(
    api_key_id: Uuid,
    version: i16,
    organization_id: Uuid,
    secret: &[u8],
) -> [u8; API_KEY_HASH_SIZE] {
    let mut hasher = blake3::Hasher::new();

    hasher.update(api_key_id.as_bytes());
    hasher.update(&version.to_le_bytes());
    hasher.update(organization_id.as_bytes());
    hasher.update(secret);

    let hash = hasher.finalize();
    let mut hash_bytes = [0u8; API_KEY_HASH_SIZE];
    hash_bytes.copy_from_slice(hash.as_bytes());

    hash_bytes
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateApiKeyInput {
    pub organization_id: Uuid,
    pub name: String,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ApiKeyWithToken {
    pub api_key: ApiKey,
    pub token: String,
}

fn generate_api_key(input: CreateApiKeyInput) -> ApiKeyWithToken {
    let api_key_id = Uuid::now_v7();
    let version = 1;

    let mut rng = ThreadRng::default();
    let mut token_data = [0u8; 16 + API_KEY_SECRET_SIZE];
    token_data[..16].copy_from_slice(api_key_id.as_bytes());
    rng.fill_bytes(&mut token_data[16..]);

    let hash = hash_api_key(
        api_key_id,
        version,
        input.organization_id,
        &token_data[16..],
    );

    let mut token = base32::encode(Alphabet::Rfc4648 { padding: false }, &token_data).to_lowercase();
    token.insert_str(0, "_");
    token.insert_str(0, "v1");
    token.insert_str(0, "_");
    token.insert_str(0, API_KEY_PREFIX);

    token_data.zeroize();

    let now = Utc::now();
    let api_key = ApiKey {
        id: api_key_id.to_string(),
        created_at: now,
        updated_at: now,
        name: input.name,
        expires_at: input.expires_at,
        version,
        secret_hash: hash,
        organization_id: input.organization_id,
    };

    return ApiKeyWithToken { api_key, token };
}

fn verify_api_key(api_key: &ApiKey, token: &str) -> bool {
    let expected_prefix = format!("{}_v{}_", API_KEY_PREFIX, api_key.version);

    if !token.starts_with(&expected_prefix) {
        return false;
    }

    let base32_part = &token[expected_prefix.len()..];
    let base32_part_upper = base32_part.to_uppercase();
    let token_data = match base32::decode(Alphabet::Rfc4648 { padding: false }, &base32_part_upper) {
        Some(data) => data,
        None => return false,
    };

    if token_data.len() != 16 + API_KEY_SECRET_SIZE {
        return false;
    }

    let token_api_key_id = match Uuid::from_slice(&token_data[..16]) {
        Ok(uuid) => uuid,
        Err(_) => return false,
    };

    if token_api_key_id.to_string() != api_key.id {
        return false;
    }

    if let Some(expiry) = api_key.expires_at {
        if Utc::now() > expiry {
            return false;
        }
    }

    let computed_hash = hash_api_key(
        token_api_key_id,
        api_key.version,
        api_key.organization_id,
        &token_data[16..],
    );

    computed_hash.ct_eq(&api_key.secret_hash).into()
}

fn main() {
    let input = CreateApiKeyInput {
        organization_id: Uuid::now_v7(),
        name: "My First API Key".to_string(),
        expires_at: None,
    };

    let api_key_with_token = generate_api_key(input);

    println!("API Key ID: {}", api_key_with_token.api_key.id);
    println!("API Key Token: {}", api_key_with_token.token);

    let is_valid = verify_api_key(&api_key_with_token.api_key, &api_key_with_token.token);
    println!("Is the API Key valid? {}", is_valid);

    let is_valid_fake = verify_api_key(&api_key_with_token.api_key, "invalid_token");
    println!("Is the fake API Key valid? {}", is_valid_fake);
}
