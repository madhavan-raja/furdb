use actix_web::{http::StatusCode, post, web, HttpResponse};
use furdb_core::models as core_models;

use crate::models;

#[post("/{database_id}")]
pub(crate) async fn create_database_handler(
    data: web::Data<core_models::furdb::FurDB>,
    path: web::Path<String>,
    create_database_params: web::Json<models::params::create_database_params::CreateDatabaseParams>,
) -> Result<HttpResponse, models::errors::ErrorResponse> {
    let database_id = path.into_inner();
    let database_name = create_database_params.get_database_name();

    let furdb = data.as_ref();
    furdb.create_database(&database_id, database_name.as_deref())?;

    Ok(HttpResponse::new(StatusCode::CREATED))
}
