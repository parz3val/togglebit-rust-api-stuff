use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone)]
pub struct AddVerifiedEmailsForm {
    pub emails: Vec<String>,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct VerifiedEmail {
    pub id: uuid::Uuid,
    pub email: String,
    pub added_by: uuid::Uuid,
}

pub fn parse_verified_emails(form: AddVerifiedEmailsForm, added_by: uuid::Uuid) -> Vec<VerifiedEmail> {
    form.emails.into_iter().map(|email| VerifiedEmail {
        id: uuid::Uuid::new_v4(),
        email,
        added_by
    }).collect()
}
