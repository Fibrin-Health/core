

use axum::extract::{FromRef, State};
use axum::routing::{delete, post};
use axum::{Json, Router};
use axum::extract::Path;

use crate::ctx::{self, Ctx};
use crate::model::{ModelController, Ehr, EhrForCreate};
use crate::{Error, Result};

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

pub fn routes(mc: ModelController) -> Router {
    let app_state = AppState { mc };

    Router::new()
        .route("/ehrs", post(create_ehr).get(list_ehrs))
        .route("/ehrs/:id", delete(delete_ehr))
        .with_state(app_state)
}

async fn create_ehr(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ehr_fc): Json<EhrForCreate>,
) -> Result<Json<Ehr>> {
    print!("->> {:<12} - create_ehr", "HANDLER");

    let ehr = mc.create_ehr(ctx, ehr_fc).await?;

    Ok(Json(ehr))

}

async fn list_ehrs(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ehr>>> {
    print!("->> {:<12} - list_ehrs", "HANDLER");

    let ehrs = mc.list_ehr(ctx).await?;

    Ok(Json(ehrs))
}

async fn delete_ehr(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ehr>> {
    print!("->> {:<12} - delete_ehr", "HANDLER");

    let ehr = mc.delete_ehr(ctx, id).await?;

    Ok(Json(ehr))
}