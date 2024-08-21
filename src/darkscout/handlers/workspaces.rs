use crate::darkscout::types::invitations::InvitationType;
use crate::darkscout::types::member::MemberRole;
use crate::darkscout::types::store::{InvitationsStore, MemberStore, UserStore, WorkspaceStore};
use crate::darkscout::types::user::{MemberProfile, UserData, UserRecord};
use crate::darkscout::types::workspace::{
    AcceptMemberInvitationForm, EditWorkspaceForm, InvitationResultStatus,
    MemberInvitationResponseData, MemberInvitationsForm, MemberWorkspace, Workspace, WorkspaceForm,
};
use crate::darkscout::types::{AppState, DSResponse};
use crate::darkscout::web::{json_error, json_resp};
use crate::{json_err, unwrap_or_else_string};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Extension, Json};
use sqlx::Error;
use std::cmp::PartialEq;
use uuid::Uuid;

pub async fn edit_workspace(
    State(state): State<AppState>,
    Extension(user_data): Extension<UserData>,
    Extension(workspace): Extension<MemberWorkspace>,
    Json(form): Json<EditWorkspaceForm>,
) -> impl IntoResponse {
    let Ok(w) = state
        .db
        .workspace
        .get_member_workspace(workspace.member_id)
        .await
    else {
        return json_err!();
    };
    let permission: bool = workspace.id != form.id;
    let wks = w.clone();
    if !permission {
        return json_err!(StatusCode::UNAUTHORIZED, "Unauthorized");
    }
    let record = Workspace {
        id: form.id,
        title: unwrap_or_else_string!(form.title, w.title),
        details: unwrap_or_else_string!(form.details, w.details),
        created_by: w.created_by,
    };
    let Ok(_) = state.db.workspace.update_workspace(record).await else {
        return json_err!();
    };
    return json_resp::<Workspace>(None, wks);
}

pub async fn create_workspace(
    State(state): State<AppState>,
    Extension(user_data): Extension<UserData>,
    Json(form): Json<WorkspaceForm>,
) -> impl IntoResponse {
    let record = Workspace {
        id: Uuid::new_v4(),
        title: form.title,
        details: form.details,
        created_by: user_data.id,
    };
    return match state.db.workspace.create_workspace(record).await {
        Ok(w) => json_resp(None, Some(w)),
        Err(e) => {
            tracing::debug!("Failed to create workspace {}", e);
            json_err!()
        }
    };
}

fn create_user_records(data: MemberInvitationsForm) -> Vec<UserRecord> {
    data.invitations
        .iter()
        .map(|r| UserRecord {
            id: Uuid::new_v4(),
            first_name: r.first_name.clone(),
            created_at: None,
            updated_at: None,
            last_name: r.last_name.clone(),
            username: r.email.clone(),
            email: r.email.clone(),
            ip_str: "".to_string(),
            password: None,
        })
        .collect()
}

pub async fn invite_new_members(
    State(state): State<AppState>,
    Extension(member): Extension<MemberProfile>,
    Json(form): Json<MemberInvitationsForm>,
) -> impl IntoResponse {
    let permission = member.role == MemberRole::ADMIN;
    if !permission {
        return json_err!(StatusCode::UNAUTHORIZED, "Unauthorized!");
    };
    let user_records = create_user_records(form);
    let mut resp_data: Vec<MemberInvitationResponseData> = vec![];
    for user in user_records {
        let result = match state.db.user.create_user(&user).await {
            Ok(_) => InvitationResultStatus::SUCCESS,
            Err(_) => InvitationResultStatus::FAILURE,
        };
        resp_data.push(MemberInvitationResponseData {
            email: user.email,
            status: result,
        });
    }
    return json_resp::<Vec<MemberInvitationResponseData>>(None, resp_data);
}

pub async fn accept_member_invitation(
    State(state): State<AppState>,
    Extension(user_data): Extension<UserData>,
    Extension(workspace): Extension<MemberWorkspace>,
    Json(form): Json<AcceptMemberInvitationForm>,
) -> impl IntoResponse {
    let Ok(inv) = state
        .db
        .invitation
        .get_verification_data(&form.user_id)
        .await
    else {
        return json_err!();
    };
    json_resp::<&str>(None, "Success")
}
