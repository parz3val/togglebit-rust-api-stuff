use crate::darkscout::types::auth::{
    NotificationStatus, PasswordResetRequest, RequestResetLinkForm, ResetPasswordWithToken,
};
use crate::darkscout::types::store::{AuthStore, UserStore};
use crate::darkscout::types::user::UserRecord;
use crate::darkscout::types::AppState;
use crate::darkscout::utils::auth::encrypt_password;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use bcrypt::BcryptError;
use sqlx::Error;
use std::future::Future;
use uuid::Uuid;
use crate::darkscout::web::{json_resp, json_error};
use crate::json_err;

pub async fn request_password_reset_link(
    State(state): State<AppState>,
    Json(form): Json<RequestResetLinkForm>,
) -> impl IntoResponse {
    return match state.db.user.get_by_email(form.email).await {
        Ok(user) => {
            let change_request = PasswordResetRequest {
                id: Uuid::new_v4(),
                token: "".to_owned(),
                expires_after: 15,
                created_at: None,
                update_at: None,
                status: NotificationStatus::CREATED,
                user_id: user.id,
            };
            tokio::spawn(async move {
                // insert the request in the into the database
                match state
                    .db
                    .auth
                    .create_password_reset_request(change_request.clone())
                    .await
                {
                    Ok(_) => {
                        // send email link to the user
                        tracing::debug!("Sending email to the user")
                    }
                    Err(e) => {
                        tracing::debug!("Failed to insert password_reset_request {}", e);
                    }
                }
            });
            (StatusCode::OK, "Sent reset email successfully")
        }
        Err(e) => {
            tracing::debug!("DS: 40144 User not found {}", e);
            (StatusCode::NOT_FOUND, "Not found")
        }
    };
}

pub async fn reset_password_with_token(
    State(state): State<AppState>,
    Json(form): Json<ResetPasswordWithToken>,
) -> impl IntoResponse {
    let new_password_hash = match encrypt_password(form.new_password) {
        Ok(h) => h,
        Err(e) => {
            tracing::debug!("Failed to encrypt password: {}", e);
            return json_err!();
        }
    };
    if let Err(e) = verify_token(form.token, form.user_id, &state).await {
        tracing::debug!("TokenVerificationError: {}", e);
        return json_err!();
    };
    if let Err(e) = state.db.user.reset_user_password(new_password_hash, form.user_id).await {
        tracing::debug!("ResetPasswordError: {}", e);
        return json_err!();
    };
    return json_resp::<&str>(None, "Success");
}

type TokenVerificationError = &'static str;
async fn verify_token(
    token: String,
    user_id: Uuid,
    state: &AppState,
) -> Result<(), TokenVerificationError> {
    return match state
        .db
        .auth
        .get_reset_request(token.clone(), user_id)
        .await
    {
        Ok(r) => {
            if r.token == token && r.expires_after != 0 {
                return Ok(());
            }
            Err("Failed to verify password token")
        }
        Err(_) => Err("Failed to verify password token"),
    };
}
