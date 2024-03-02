use actix_web::{post, web, Responder};
use furdb_core::{Column, DataType, Database, TableInfo};
use std::error::Error;

use crate::utils::get_database_path;

use crate::models;

#[post("/{database_id}/{table_id}")]
pub async fn create_table_handler(
    path: web::Path<(String, String)>,
    // req: HttpRequest,
    table_generatable: Option<web::Json<models::TableGenerator>>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();
    // let params = web::Query::<models::TableParams>::from_query(req.query_string()).unwrap();

    let database = Database::get_database(get_database_path(&database_id))?;

    let table_info = table_generatable
        .map(|table_generatable| generate_table_info(table_generatable.clone()).unwrap());

    let tb = database.get_table(&table_id, table_info)?;

    let info = tb.get_info()?.clone();
    let res = models::TableResponse::new(info);

    Ok(web::Json(res))
}

pub(crate) fn generate_table_info(
    table_info_generatable: models::TableGenerator,
) -> Result<TableInfo, Box<dyn Error>> {
    let columns = table_info_generatable.columns.map(|column_generators| {
        column_generators
            .iter()
            .map(|column_generatable| generate_column(column_generatable.clone()).unwrap())
            .collect()
    });

    TableInfo::new(
        &table_info_generatable.name,
        table_info_generatable.converter_server.as_deref(),
        columns,
    )
}

pub(crate) fn generate_column(
    column_generatable: models::ColumnGenerator,
) -> Result<Column, Box<dyn Error>> {
    Column::new(
        &column_generatable.id,
        column_generatable.description.as_deref(),
        column_generatable.size,
        generate_data_type(column_generatable.data_type)?,
    )
}

pub(crate) fn generate_data_type(
    data_type_generatable: models::DataTypeGenerator,
) -> Result<DataType, Box<dyn Error>> {
    DataType::new(
        data_type_generatable.id.as_str(),
        data_type_generatable.converter_endpoint_override.as_deref(),
    )
}
