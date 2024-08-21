use crate::darkscout::types::member::MemberRole;
use crate::darkscout::types::store::{VerifiedEmailsRepo, VerifiedEmailsStore};
use crate::darkscout::types::user::{MemberProfile, UserData};
use crate::darkscout::types::verified_emails::{parse_verified_emails, AddVerifiedEmailsForm};
use crate::darkscout::types::{AppState, DSResponse};
use crate::darkscout::web::{json_error, json_resp};
use crate::{json_err, unwrap_or_else_string};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Extension, Json};
use sqlx::Error;
use uuid::Uuid;

pub async fn add_verified_emails(
    State(state): State<AppState>,
    Extension(member): Extension<MemberProfile>,
    Json(form): Json<AddVerifiedEmailsForm>,
) -> impl IntoResponse {
    let permission = member.role == MemberRole::ADMIN;
    if !permission {
        return json_err!(StatusCode::UNAUTHORIZED, "Unauthorized!");
    };
    tracing::debug!("The member found is {:?}", member);
    let parsed_emails = parse_verified_emails(form, member.id);
    let db = state.db;
    match db
        .verified_emails
        .add_verified_emails(parsed_emails.clone())
        .await
    {
        Ok(_) => {
            // Send the verification email in the background
            tokio::spawn(async move {
                println!("This is background task");
            });
        }
        Err(e) => {
            tracing::debug!("Failed to add verified emails {}", e);
            return json_err!(StatusCode::INTERNAL_SERVER_ERROR, "Failed to add emails");
        }
    }
    return json_resp(Some(StatusCode::OK), parsed_emails);
    // return json_err!(StatusCode::NOT_FOUND, "Not implemented!");
}
