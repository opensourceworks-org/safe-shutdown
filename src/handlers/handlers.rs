use crate::safener::safener::Safener;
use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct StateSwitch {
    pub safe: bool,
}

pub async fn get_state_handler(State(safener): State<Arc<Safener>>) -> impl IntoResponse {
    let mut shutdownable_status = HashMap::new();
    let safe = safener.is_safe();
    shutdownable_status.insert("shutdownable", &safe);
    (StatusCode::OK, Json(shutdownable_status)).into_response()
}

pub async fn change_state_handler(
    State(safener): State<Arc<Safener>>,
    Query(params): Query<StateSwitch>,
) -> impl IntoResponse {
    let safe = params.safe;

    if safe {
        safener.set_safe().unwrap();
    } else {
        safener.set_unsafe().unwrap();
    }
    (StatusCode::OK, Json(safe)).into_response()
}
