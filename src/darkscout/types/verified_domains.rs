
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone)]
pub struct AddVerifiedDomainsForm {
    pub domains: Vec<String>,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct VerifiedDomain {
    pub id: uuid::Uuid,
    pub domain: String,
    pub added_by: uuid::Uuid,
}

pub fn parse_verified_domains(form: AddVerifiedDomainsForm, added_by: uuid::Uuid) -> Vec<VerifiedDomain> {
    form.domains.into_iter().map(|domain| VerifiedDomain {
        id: uuid::Uuid::new_v4(),
        domain,
        added_by
    }).collect()
}
