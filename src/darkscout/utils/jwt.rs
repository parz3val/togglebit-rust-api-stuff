use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, TokenData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims<T>
{
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub payload: T,
}


pub fn encode_jwt<T>(
    header: &Header,
    claims: &T,
    key: &EncodingKey,
) -> Result<String, jsonwebtoken::errors::Error>
    where
        T: Serialize + for<'de> Deserialize<'de>,
{
    let token = encode(header, claims, key)?;
    Ok(token)
}

pub fn decode_jwt<'a, T>(
    token: String,
    key: &DecodingKey,
    validation: &Validation,
) -> Result<TokenData<T>, jsonwebtoken::errors::Error>
    where
        T: for<'de> Deserialize<'de> + Serialize,
{
    let token_data = decode::<T>(token.as_str(), key, validation)?;
    Ok(token_data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode_jwt(){

    }
    #[test]
    fn decode_jwt(){

    }
}