use crate::darkscout::adapters::dsbreach::BreachName;
use crate::darkscout::adapters::{DSProvider, DarkSearchProvider};
use crate::darkscout::types::darkmonitor::{
    BreachInfo, DomainStats, EmailStats, EmailStatsConverter,
};
use crate::darkscout::types::{AppState, DSResponse};
use crate::darkscout::utils::countries::get_country_code_from_name;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn get_stats_by_email(
    State(state): State<AppState>,
    Path(email): Path<String>,
) -> impl IntoResponse {
    match state.ds_provider.get_stats_by_email(email.as_str()).await {
        Ok(result) => {
            return (
                StatusCode::OK,
                Json(DSResponse {
                    data: Some(EmailStats {
                        breach_info: build_breach_info(result),
                    }),
                    err: None,
                }),
            );
        }
        Err(err) => {
            tracing::debug!("{}", err);
            return (
                StatusCode::NOT_FOUND,
                Json(DSResponse {
                    data: None,
                    err: Some("Your email wasn't found in the darkweb or any hacked lists."),
                }),
            );
        }
    };
}

pub async fn get_stats_by_domain(
    State(state): State<AppState>,
    Path(domain): Path<String>,
) -> impl IntoResponse {
    return match state
        .ds_provider
        .get_stats_by_email(format!("a@{}", domain).as_str())
        .await
    {
        Ok(result) => (
            StatusCode::OK,
            Json(DSResponse {
                data: Some(DomainStats {
                    breach_info: build_breach_info(result),
                }),
                err: None,
            }),
        ),
        Err(err) => {
            tracing::debug!("Error: {}", err);
            (
                StatusCode::NOT_FOUND,
                Json(DSResponse {
                    data: None,
                    err: Some(
                        "No data found. Your wasn't domain hasn't been found in the darkweb.",
                    ),
                }),
            )
        }
    };
}

fn build_breach_info(info: Vec<BreachName>) -> Vec<BreachInfo> {
    return info
        .iter()
        .map(|x| BreachInfo {
            name: String::from(&x.name),
            title: String::from(&x.title),
            description: String::from(&x.description),
            logo_path: String::from(&x.logo_path),
            domain: x.domain.clone(),
            breach_date: x.breach_date.clone(),
            added_date: x.added_date.clone(),
            modified_date: x.modified_date.clone(),
            pwn_count: x.pwn_count,
            data_classes: x.data_classes.clone(),
            is_verified: x.is_verified,
            is_fabricated: x.is_fabricated,
            is_sensitive: x.is_sensitive,
            is_retired: x.is_retired,
            is_spamlist: x.is_spamlist,
            is_malicious_verified: x.is_malicious_verified,
            is_subscription_free: x.is_subscription_free,
            country_code: String::from(get_country_code_from_name(&x.name)),
        })
        .collect();
}

pub async fn get_analytics_by_email(
    State(state): State<AppState>,
    Path(email): Path<String>,
) -> impl IntoResponse {
    return match state.ds_provider.get_stats_by_email(email.as_str()).await {
        Ok(result) => (
            StatusCode::OK,
            Json(DSResponse {
                data: Some(
                    EmailStats {
                        breach_info: build_breach_info(result),
                    }
                    .convert_to_email_analytics(),
                ),
                err: None,
            }),
        ),
        Err(err) => {
            tracing::debug!("{}", err);
            (
                StatusCode::NOT_FOUND,
                Json(DSResponse {
                    data: None,
                    err: Some("Your email wasn't found in the darkweb or any hacked lists."),
                }),
            )
        }
    };
}

pub async fn get_dark_search_by_email(
    State(state): State<AppState>,
    Path(email): Path<String>,
) -> impl IntoResponse {
    match state
        .ds_darkengine_provider
        .get_stats_by_email(email.as_str())
        .await
    {
        Ok(result) => {
            return (
                StatusCode::OK,
                Json(DSResponse {
                    data: Some(result),
                    err: None,
                }),
            );
        }
        Err(err) => {
            tracing::debug!("{}", err);
            return (
                StatusCode::NOT_FOUND,
                Json(DSResponse {
                    data: None,
                    err: Some("Your email wasn't found in the darkweb or any hacked lists."),
                }),
            );
        }
    }
}

pub async fn get_dark_search_by_domain(
    State(state): State<AppState>,
    Path(domain): Path<String>,
) -> impl IntoResponse {
    match state
        .ds_darkengine_provider
        .get_stats_by_domain(domain.as_str())
        .await
    {
        Ok(result) => {
            return (
                StatusCode::OK,
                Json(DSResponse {
                    data: Some(result),
                    err: None,
                }),
            );
        }
        Err(err) => {
            tracing::debug!("{}", err);
            return (
                StatusCode::NOT_FOUND,
                Json(DSResponse {
                    data: None,
                    err: Some("Your email wasn't found in the darkweb or any hacked lists."),
                }),
            );
        }
    }
}
