use crate::darkscout::types::auth::LoggedInUserClaims;
use crate::darkscout::types::invitations::InvitationStatus;
use crate::darkscout::types::member::{MemberData, MemberRecord, MemberRole, MemberStatus};
use crate::darkscout::types::user::{
    Auth, MemberProfile, PasswordResetForm, UserData, UserEditForm, UserSignupResponseData,
};
use crate::darkscout::types::workspace::{MemberWorkspace, MemberWorkspaceDetails};
use crate::darkscout::types::{errors, DSResponse};
use crate::darkscout::types::{
    invitations::new_verification_code,
    store::{InvitationsStore, MemberStore, UserStore, WorkspaceStore},
    user::{LoginForm, LoginResponse, UserRecord, UserSignupForm, VerifyUserForm},
    workspace::Workspace,
    AppState,
};
use crate::darkscout::utils::auth::{encrypt_password, verify_password};
use crate::darkscout::utils::jwt::{encode_jwt, TokenClaims};
use axum::{debug_handler, extract::State, Extension, Json};
use chrono::{DateTime, Local, Utc};
use jsonwebtoken::{EncodingKey, Header};
use uuid::Uuid;

use crate::darkscout::adapters::smtp_mailer::send_smtp_email;
use crate::darkscout::web::{json_error, json_resp};
use crate::{json_err, unwrap_or_else_string};
use axum::body::Body;
use axum::extract::Request;
use axum::http::{Extensions, StatusCode};
use axum::response::IntoResponse;
use bcrypt::BcryptError;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use std::time::Instant;

pub fn get_auth_tokens(
    member_record: &Option<MemberData>,
    user_record: UserRecord,
    default_workspace: Option<&MemberWorkspace>,
    jwt_secret: String,
    role: MemberRole,
) -> Option<Auth> {
    let workspace: Option<MemberWorkspace> = match default_workspace {
        None => None,
        Some(w) => Some(MemberWorkspace {
            id: w.id,
            title: w.title.clone(),
            details: w.details.clone(),
            created_by: w.created_by,
            workspace_order: w.workspace_order,
            is_default: w.is_default,
            member_id: w.member_id,
        }),
    };
    let member_profile = match member_record {
        Some(d) => Some(MemberProfile {
            id: d.id,
            user_id: d.user_id,
            first_name: user_record.first_name.clone(),
            last_name: user_record.last_name.clone(),
            email: user_record.email.clone(),
            role: MemberRole::ADMIN,
            verified_domains: None,
            created_at: d.created_at,
            updated_at: d.updated_at,
        }),
        None => None,
    };
    let user_profile = UserData {
        id: user_record.id,
        first_name: user_record.first_name,
        last_name: user_record.last_name,
        username: user_record.email.clone(),
        email: user_record.email,
    };

    let access_claim = LoggedInUserClaims {
        workspace,
        member: member_profile,
        user: Some(user_profile.clone()),
    };
    let ts = Local::now().to_utc().timestamp();
    tracing::debug!("TIMESTAMPis : {}", ts);
    let access_claim_data = TokenClaims {
        sub: "access-token".to_string(),
        iat: ts as usize,
        exp: (ts + (60 * 60 * 24) as i64) as usize, // 24 hour in ms
        payload: access_claim,
    };

    let auth_claim_data = TokenClaims {
        sub: "auth-token".to_string(),
        iat: ts as usize,
        exp: (ts + (60 * 60 * 24) as i64) as usize, // 24 hour in ms
        payload: user_profile.clone(),
    };
    let refresh_claim_data = TokenClaims {
        sub: "auth-token".to_string(),
        iat: ts as usize,
        exp: (ts + (60 * 60 * 24 * 7) as i64) as usize, // one week in ms
        payload: user_profile,
    };
    let key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let auth_token =
        encode_jwt(&Header::default(), &auth_claim_data, &key).unwrap_or_else(|_| "".to_string());
    let access_token =
        encode_jwt(&Header::default(), &access_claim_data, &key).unwrap_or_else(|_| "".to_string());
    let refresh_token = encode_jwt(&Header::default(), &refresh_claim_data, &key)
        .unwrap_or_else(|_| "".to_string());
    Some(Auth {
        auth_token,
        refresh_token,
        access_token,
    })
}

pub async fn log_in(
    State(state): State<AppState>,
    Json(_form): Json<LoginForm>,
) -> impl IntoResponse {
    let db = state.db;

    let jwt_secret: String = state.settings.jwt.jwt_secret;

    let Ok(user_record) = db.user.get_by_email(_form.email).await else {
        tracing::debug!("UserNotFound: Error while querying user.");
        return json_err!();
    };
    let member_record = match db.member.get_member_profile(user_record.id).await {
        Ok(d) => Some(d),
        Err(e) => {
            tracing::debug!("Failed to get member profile {}", e);
            None
        }
    };
    let Some(hashed_password) = user_record.clone().password else {
        tracing::debug!("NoPassword: No password provided.");
        return json_err!();
    };

    let Ok(result) = verify_password(_form.password, hashed_password) else {
        return json_err!(StatusCode::UNAUTHORIZED, "Password didn't match");
    };

    let Ok(workspaces) = db
        .workspace
        .get_user_workspaces(user_record.clone().id)
        .await
    else {
        return json_err!();
    };

    let default_workspace = workspaces.iter().filter(|x| x.is_default).map(|x| x).next();

    // let before = Instant::now();
    let auth = get_auth_tokens(
        &member_record,
        user_record.clone(),
        default_workspace,
        jwt_secret,
        MemberRole::ADMIN,
    )
    .unwrap_or_else(|| Auth {
        access_token: "".to_string(),
        refresh_token: "".to_string(),
        auth_token: "".to_string(),
    });

    let verified_domains: Vec<String> = vec!["krispcall.com".to_string()];

    let member_profile = match member_record {
        Some(d) => Some(MemberProfile {
            id: d.id,
            user_id: d.user_id,
            first_name: user_record.first_name,
            last_name: user_record.last_name,
            email: user_record.email,
            role: MemberRole::ADMIN,
            created_at: d.created_at,
            updated_at: d.updated_at,
            verified_domains: Some(verified_domains),
        }),
        None => None,
    };
    let data = LoginResponse {
        workspaces,
        member_profile: member_profile,
        auth,
    };
    // println!("Time it took to process request {:.2?} ", before.elapsed());
    return json_resp::<LoginResponse>(Some(StatusCode::OK), data);
    // return json_err!();
}

pub async fn verify_user(
    State(state): State<AppState>,
    Json(_form): Json<VerifyUserForm>,
) -> impl IntoResponse {
    let db = state.db;

    let invitation = match db.invitation.get_verification_data(&_form.id).await {
        Ok(code) => code,
        Err(e) => {
            tracing::debug!(" Failed to get the verification data {}", e);
            return json_err!();
        }
    };

    if invitation.details != _form.code {
        // TODO: Provide correct message to frontend as well
        return json_err!();
    }

    if let Err(e) = db
        .invitation
        .update_invitation_status(InvitationStatus::DELETED, _form.id)
        .await
    {
        tracing::debug!("UpdateInvitationFailed {}", e);
        return json_err!();
    };

    // TODO: Move this to task queue in the future
    tokio::spawn(async move {
        let ts: DateTime<Utc> = Local::now().to_utc();
        let member = MemberRecord {
            id: Uuid::new_v4(),
            joined_at: ts,
            profile_picture: Some(String::from("profile://")),
            status: Some(MemberStatus::VERIFIED),
            role: Some(MemberRole::ADMIN),
            invitation_token: Some(String::from("NA")),
            invitation_code: Some(0),
            user_id: _form.id,
            invited_by: invitation.created_by,
            role_granted_by: Some(_form.id),
            workspace: invitation.workspace,
        };
        db.member
            .create_new_member(member.clone())
            .await
            .unwrap_or_else(|e| tracing::debug!("{}", e));

        db.workspace
            .create_member_workspace_details(MemberWorkspaceDetails {
                id: invitation.workspace,
                workspace_order: 0,
                is_default: true,
                is_fav: false,
                member_id: member.id,
            })
            .await
            .unwrap_or_else(|e| tracing::debug!("{}", e));
    });
    return json_resp::<&str>(None, "Successfully verified user");
}

pub async fn sign_up(
    State(state): State<AppState>,
    Json(_form): Json<UserSignupForm>,
) -> impl IntoResponse {
    let db = state.db;

    let Ok(user_record) = UserRecord::try_from(_form) else {
        return json_err!(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create user. Try again letter."
        );
    };
    // Create user
    let Ok(id) = db.user.create_user(&user_record).await else {
        return json_err!(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to insert user. Please try again with another email."
        );
    };
    // Create workspace
    let workspace = match db
        .workspace
        .create_workspace(match Workspace::try_from(user_record.clone()) {
            Ok(w) => w,
            Err(e) => {
                tracing::debug!("Failed to create user workspace {}", e);
                return json_err!(
                    StatusCode::BAD_REQUEST,
                    "Failed to create workspace object."
                );
            }
        })
        .await
    {
        Ok(w) => w,
        Err(e) => {
            tracing::debug!("Failed to insert workspace into db {}", e);
            return json_err!(StatusCode::BAD_REQUEST, "Failed to insert workspace");
        }
    };

    // Create invitation
    let verification_email = new_verification_code(
        user_record.id,
        user_record.id,
        workspace.id,
        user_record.email.clone(),
    );

    if let Err(e) = db.invitation.create_new(verification_email.clone()).await {
        tracing::debug!("InvitationCreationError : {}", e);
        return json_err!(StatusCode::BAD_REQUEST, "Failed to send invitation");
    };
    // send smtp email
    let ve = verification_email.clone();
    tokio::spawn(async move {
        if let Err(e) = send_smtp_email(
            String::from("iamtheparzival@gmail.com"),
            ve.email.clone(),
            ve.msg,
            ve.subject.as_str(),
            &state.smtp_client,
        )
        .await
        {
            tracing::debug!("SMTPError : {}", e);
        }
    });

    json_resp(
        None,
        UserSignupResponseData {
            first_name: user_record.first_name,
            last_name: user_record.last_name,
            id: user_record.id,
            email: user_record.email,
        },
    )
}

pub async fn change_password(
    Extension(user_data): Extension<UserData>,
    State(state): State<AppState>,
    Json(reset_form): Json<PasswordResetForm>,
) -> impl IntoResponse {
    let password_hash = match state.db.user.get_by_email(user_data.email).await {
        Ok(user) => match user.password {
            Some(password) => password,
            None => {
                return json_err!();
            }
        },
        Err(_) => {
            return json_err!();
        }
    };
    let Ok(password_match) = verify_password(reset_form.old_password, password_hash.clone()) else {
        return json_err!();
    };
    if !password_match {
        return json_error(
            Some(StatusCode::UNAUTHORIZED),
            Some("username/password mismatch"),
        );
    }
    let Ok(new_password_hash) = encrypt_password(reset_form.new_password) else {
        return json_err!();
    };

    match state
        .db
        .user
        .reset_user_password(new_password_hash, user_data.id)
        .await
    {
        Ok(_) => return json_resp(None, "Successfully updated password"),
        Err(_) => (return json_err!()),
    }
}

pub async fn edit_user_profile(
    Extension(user_data): Extension<UserData>,
    Extension(workspace): Extension<MemberWorkspace>,
    State(state): State<AppState>,
    Json(new_user): Json<UserEditForm>,
) -> impl IntoResponse {
    let Ok(user) = state.db.user.get_by_id(user_data.id).await else {
        return json_error::<&'static str>(None, None);
    };

    let record = UserRecord {
        id: user.id,
        first_name: unwrap_or_else_string!(new_user.first_name, user.first_name),
        last_name: unwrap_or_else_string!(new_user.last_name, user.last_name),
        username: user.username,
        email: user.email,
        ip_str: user.ip_str,
        password: user.password,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    if let Err(e) = state.db.user.update_user(record).await {
        tracing::debug!("UserUpdateFailed : {}", e);
        return json_error::<&'static str>(None, None);
    };

    // update member profile picture if it exists
    if let Some(url) = new_user.profile_picture {
        tokio::spawn(async move {
            state
                .db
                .member
                .update_member_profile_picture(workspace.member_id, url)
                .await
                .unwrap_or_else(|e| {
                    tracing::debug!("Failed to update member profile picture: {}", e);
                });
        });
    }
    return json_resp::<&'static str>(None, "Success");
}
