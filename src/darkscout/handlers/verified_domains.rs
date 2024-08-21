use crate::darkscout::types::member::MemberRole;
use crate::darkscout::types::store::{VerifiedDomainRepo, VerifiedDomainsStore};
use crate::darkscout::types::user::{MemberProfile, UserData};
use crate::darkscout::types::verified_domains::{
    parse_verified_domains,
    AddVerifiedDomainsForm
};
use crate::darkscout::types::{AppState, DSResponse};
use crate::darkscout::web::{json_error, json_resp};
use crate::{json_err, unwrap_or_else_string};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Extension, Json};
use sqlx::Error;
use uuid::Uuid;

pub async fn add_verified_domains(
    State(state): State<AppState>,
    Extension(member): Extension<MemberProfile>,
    Json(form): Json<AddVerifiedDomainsForm>,
) -> impl IntoResponse {
    let permission = member.role == MemberRole::ADMIN;
    if !permission {
        return json_err!(StatusCode::UNAUTHORIZED, "Unauthorized!");
    };
    tracing::debug!("The member found is {:?}", member);
    let parsed_domains = parse_verified_domains(form, member.id);
    let db = state.db;
    match db
        .verified_domains
        .add_verified_domains(parsed_domains.clone())
        .await
    {
        Ok(_) => {}
        Err(e) => {
            tracing::debug!("Failed to add verified domains {}", e);
            return json_err!(StatusCode::INTERNAL_SERVER_ERROR, "Failed to add domains");
        }
    }
    return json_resp(Some(StatusCode::OK), parsed_domains);
    // return json_err!(StatusCode::NOT_FOUND, "Not implemented!");
}
