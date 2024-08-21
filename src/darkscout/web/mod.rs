use crate::darkscout::types::DSResponse;
use axum::http::StatusCode;
use axum::Json;

pub fn json_error<T>(code: Option<StatusCode>, msg: Option<&'static str>) -> (StatusCode, Json<DSResponse<T>>) {
    let code = code.unwrap_or_else(|| StatusCode::OK);
    let msg = msg.unwrap_or_else(|| "Internal Server Error");
    (
        code,
        Json(DSResponse {
            data: None,
            err: Some(msg),
        }),
    )
}

pub fn json_resp<T>(code: Option<StatusCode>, data: T) -> (StatusCode, Json<DSResponse<T>>) {
    (code.unwrap_or_default(), Json(DSResponse { data: Some(data), err: None }))
}
