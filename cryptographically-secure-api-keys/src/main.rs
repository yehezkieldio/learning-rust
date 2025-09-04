use base32::Alphabet;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;
use uuid::Uuid;
use zeroize::Zeroize;
use std::fmt;

const API_KEY_SECRET_SIZE: usize = 32; // 256 bits
const API_KEY_HASH_SIZE: usize = 32; // 256 bits
const CHECKSUM_SIZE: usize = 4; // CRC32 checksum
const TOKEN_DATA_SIZE: usize = 16 + API_KEY_SECRET_SIZE + CHECKSUM_SIZE; // UUID + Secret + Checksum

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyConfig {
    pub prefix: String,
    pub version: i16,
    pub include_checksum: bool,
}

impl Default for ApiKeyConfig {
    fn default() -> Self {
        Self {
            prefix: "elizielx".to_string(),
            version: 1,
            include_checksum: true,
        }
    }
}

#[derive(Debug)]
pub struct ParsedToken {
    pub prefix: String,
    pub version: i16,
    pub uuid: Uuid,
    pub secret: [u8; API_KEY_SECRET_SIZE],
    pub checksum: Option<u32>,
}

impl Drop for ParsedToken {
    fn drop(&mut self) {
        self.secret.zeroize();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApiKeyError {
    InvalidPrefix,
    InvalidVersion,
    InvalidFormat,
    InvalidUuid,
    InvalidChecksum,
    Expired,
    InvalidHash,
    DecodingError,
}

impl fmt::Display for ApiKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiKeyError::InvalidPrefix => write!(f, "Invalid API key prefix"),
            ApiKeyError::InvalidVersion => write!(f, "Invalid API key version"),
            ApiKeyError::InvalidFormat => write!(f, "Invalid API key format"),
            ApiKeyError::InvalidUuid => write!(f, "Invalid UUID in token"),
            ApiKeyError::InvalidChecksum => write!(f, "Invalid checksum"),
            ApiKeyError::Expired => write!(f, "API key has expired"),
            ApiKeyError::InvalidHash => write!(f, "Invalid API key hash"),
            ApiKeyError::DecodingError => write!(f, "Failed to decode token"),
        }
    }
}

impl std::error::Error for ApiKeyError {}

#[derive(Debug, Clone, Serialize)]
pub struct ApiKey {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub organization_id: Uuid,

    #[serde(skip_serializing)]
    pub version: i16,
    #[serde(skip_serializing)]
    pub secret_hash: [u8; API_KEY_HASH_SIZE],
    #[serde(skip_serializing)]
    pub config: ApiKeyConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateApiKeyInput {
    pub organization_id: Uuid,
    pub name: String,
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub config: Option<ApiKeyConfig>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ApiKeyWithToken {
    pub api_key: ApiKey,
    pub token: String,
}

fn hash_api_key(
    api_key_id: Uuid,
    version: i16,
    organization_id: Uuid,
    secret: &[u8],
) -> [u8; API_KEY_HASH_SIZE] {
    let mut hasher = blake3::Hasher::new();

    let mut input = Vec::with_capacity(16 + 2 + 16 + secret.len());
    input.extend_from_slice(api_key_id.as_bytes());
    input.extend_from_slice(&version.to_le_bytes());
    input.extend_from_slice(organization_id.as_bytes());
    input.extend_from_slice(secret);

    hasher.update(&input);
    let hash = hasher.finalize();

    let mut hash_bytes = [0u8; API_KEY_HASH_SIZE];
    hash_bytes.copy_from_slice(hash.as_bytes());
    hash_bytes
}

fn calculate_checksum(data: &[u8]) -> u32 {
    crc32fast::hash(data)
}

fn parse_token(token: &str) -> Result<ParsedToken, ApiKeyError> {
    // Expected format: "prefix_v{version}_{base32_data}"
    let parts: Vec<&str> = token.splitn(3, '_').collect();
    if parts.len() != 3 {
        return Err(ApiKeyError::InvalidFormat);
    }

    let prefix = parts[0].to_string();

    let version_str = parts[1];
    if !version_str.starts_with('v') {
        return Err(ApiKeyError::InvalidVersion);
    }
    let version: i16 = version_str[1..].parse()
        .map_err(|_| ApiKeyError::InvalidVersion)?;

    let base32_data = parts[2].to_uppercase();
    let decoded_data = base32::decode(Alphabet::Rfc4648 { padding: false }, &base32_data)
        .ok_or(ApiKeyError::DecodingError)?;

    // Validate data length
    let expected_len = if decoded_data.len() == 16 + API_KEY_SECRET_SIZE {
        16 + API_KEY_SECRET_SIZE // Without checksum
    } else if decoded_data.len() == TOKEN_DATA_SIZE {
        TOKEN_DATA_SIZE // With checksum
    } else {
        return Err(ApiKeyError::InvalidFormat);
    };

    // Extract UUID
    let uuid = Uuid::from_slice(&decoded_data[..16])
        .map_err(|_| ApiKeyError::InvalidUuid)?;

    let mut secret = [0u8; API_KEY_SECRET_SIZE];
    secret.copy_from_slice(&decoded_data[16..16 + API_KEY_SECRET_SIZE]);

    // Extract and verify checksum if present
    let checksum = if expected_len == TOKEN_DATA_SIZE {
        let checksum_bytes = &decoded_data[16 + API_KEY_SECRET_SIZE..];
        let stored_checksum = u32::from_le_bytes([
            checksum_bytes[0], checksum_bytes[1],
            checksum_bytes[2], checksum_bytes[3]
        ]);

        let computed_checksum = calculate_checksum(&decoded_data[..16 + API_KEY_SECRET_SIZE]);
        if stored_checksum != computed_checksum {
            return Err(ApiKeyError::InvalidChecksum);
        }
        Some(stored_checksum)
    } else {
        None
    };

    Ok(ParsedToken {
        prefix,
        version,
        uuid,
        secret,
        checksum,
    })
}

impl ApiKey {
    pub fn generate(input: CreateApiKeyInput) -> ApiKeyWithToken {
        let config = input.config.unwrap_or_default();
        let api_key_id = Uuid::now_v7();

        let mut secret = [0u8; API_KEY_SECRET_SIZE];
        getrandom::fill(&mut secret).expect("Failed to generate random bytes");

        let hash = hash_api_key(
            api_key_id,
            config.version,
            input.organization_id,
            &secret,
        );

        let mut token_data = Vec::with_capacity(TOKEN_DATA_SIZE);
        token_data.extend_from_slice(api_key_id.as_bytes());
        token_data.extend_from_slice(&secret);

        // Add checksum if enabled
        if config.include_checksum {
            let checksum = calculate_checksum(&token_data);
            token_data.extend_from_slice(&checksum.to_le_bytes());
        }

        // Encode token
        let encoded = base32::encode(Alphabet::Rfc4648 { padding: false }, &token_data)
            .to_lowercase();
        let token = format!("{}_v{}_{}", config.prefix, config.version, encoded);

        secret.zeroize();

        let now = Utc::now();
        let api_key = ApiKey {
            id: api_key_id.to_string(),
            created_at: now,
            updated_at: now,
            name: input.name,
            expires_at: input.expires_at,
            version: config.version,
            secret_hash: hash,
            organization_id: input.organization_id,
            config,
        };

        ApiKeyWithToken { api_key, token }
    }

    pub fn verify(&self, token: &str) -> Result<bool, ApiKeyError> {
        let parsed = parse_token(token)?;

        if parsed.prefix != self.config.prefix {
            return Err(ApiKeyError::InvalidPrefix);
        }

        if parsed.version != self.version {
            return Err(ApiKeyError::InvalidVersion);
        }

        if parsed.uuid.to_string() != self.id {
            return Ok(false);
        }

        if let Some(expiry) = self.expires_at {
            if Utc::now() > expiry {
                return Err(ApiKeyError::Expired);
            }
        }

        // Verify hash using constant-time comparison
        let computed_hash = hash_api_key(
            parsed.uuid,
            self.version,
            self.organization_id,
            &parsed.secret,
        );

        Ok(computed_hash.ct_eq(&self.secret_hash).into())
    }

    pub fn update(&mut self, name: Option<String>, expires_at: Option<Option<DateTime<Utc>>>) {
        if let Some(new_name) = name {
            self.name = new_name;
        }
        if let Some(new_expiry) = expires_at {
            self.expires_at = new_expiry;
        }
        self.updated_at = Utc::now();
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.map_or(false, |expiry| Utc::now() > expiry)
    }

    pub fn redacted(&self) -> String {
        format!("ApiKey(id={}, name={})", self.id, self.name)
    }
}

pub fn generate_api_key(input: CreateApiKeyInput) -> ApiKeyWithToken {
    ApiKey::generate(input)
}

pub fn verify_api_key(api_key: &ApiKey, token: &str) -> bool {
    api_key.verify(token).unwrap_or(false)
}

fn main() {
    let input = CreateApiKeyInput {
        organization_id: Uuid::now_v7(),
        name: "My First API Key".to_string(),
        expires_at: None,
        config: None,
    };

    let api_key_with_token = ApiKey::generate(input);

    println!("API Key ID:             {}", api_key_with_token.api_key.id);
    println!("API Key Token:          {}", api_key_with_token.token);

    // Test verification
    match api_key_with_token.api_key.verify(&api_key_with_token.token) {
        Ok(is_valid) => println!("Is the API Key valid?   {}", is_valid),
        Err(e) => println!("Verification error:    {}", e),
    }

    // Test with invalid token
    match api_key_with_token.api_key.verify("invalid_token") {
        Ok(is_valid) => println!("Is the fake API Key valid?   {}", is_valid),
        Err(e) => println!("Verification error:     {}", e),
    }
}