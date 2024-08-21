use crate::darkscout::types::user::UserData;
use crate::darkscout::types::workspace::MemberWorkspace;
use crate::darkscout::types::AppState;
use axum::response::IntoResponse;
use axum::{extract::State, Extension};

pub async fn get_member_settings(
    State(state): State<AppState>,
    Extension(user_data): Extension<UserData>,
    Extension(workspace): Extension<MemberWorkspace>,
) -> impl IntoResponse {
    String::from("Member settings endpoint")
}
