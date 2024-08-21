use axum::http::StatusCode;
use axum::{
    body::Body,
    extract::Request,
    http,
    http::Response,
    middleware::Next,
    // response::IntoResponse,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::darkscout::types::auth::LoggedInUserClaims;
use crate::darkscout::types::user::{MemberProfile, UserData};
use crate::darkscout::types::workspace::MemberWorkspace;
use crate::darkscout::utils::jwt::{decode_jwt, TokenClaims};

// type DSCode = StatusCode;

pub async fn authorization_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| StatusCode::FORBIDDEN)?,
        None => {
            tracing::debug!("No header");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let mut header = auth_header.split_whitespace();
    let (bearer, token) = (header.next(), header.next());
    // dbg!(bearer);

    let token_data = match decode_jwt::<TokenClaims<LoggedInUserClaims>>(
        token.unwrap().to_string(),
        &DecodingKey::from_secret("randomkeysuppose".as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(data) => data,
        Err(e) => {
            tracing::debug!("DS-10005 | Error Decoding token: {}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    match token_data.claims.payload.user {
        None => {
            tracing::debug!("DS-10404 | no payload found");
            return Err(StatusCode::UNAUTHORIZED);
        }
        Some(user) => req.extensions_mut().insert(user),
    };

    match token_data.claims.payload.workspace {
        Some(w) => {
            req.extensions_mut().insert(w);
        }
        None => {
            tracing::debug!("DS-WARNING | No workspace found");
        }
    };
    match token_data.claims.payload.member {
        None => {
            tracing::debug!("DS-10404 | no member payload found");
            return Ok(next.run(req).await);
        }
        Some(member) => {
            dbg!("Member was found {:?}", member.clone());
            req.extensions_mut().insert(member)
        }
    };
    Ok(next.run(req).await)
}
