use actix_web::{get, web, HttpRequest, Responder};
use std::{error::Error, path::PathBuf};

use furdb_core::{Column, DataType, Database, DatabaseInfo, TableInfo};

use crate::config::get_fur_directory;

use crate::models;

#[get("/{database_id}/{table_id}")]
pub(crate) async fn get_table_info_handler(
    path: web::Path<(String, String)>,
    req: HttpRequest,
    table_generatable: Option<web::Json<models::CreateTableParams>>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();
    let params = web::Query::<models::TableParams>::from_query(req.query_string()).unwrap();

    let working_dir = params.working_dir.as_ref().map(|wd| PathBuf::from(wd));
    let db = get_db(working_dir, &database_id, params.db_name.clone())?;

    let table_info = table_generatable
        .map(|table_generatable| generate_table_info(table_generatable.clone()).unwrap());

    let tb = db.get_table(&table_id, table_info)?;

    let info = tb.get_info()?.clone();
    let res = models::TableResponse::new(info);

    Ok(web::Json(res))
}

pub(crate) fn get_db(
    working_dir: Option<PathBuf>,
    db_id: &str,
    db_name: Option<String>,
) -> Result<Database, Box<dyn Error>> {
    let working_dir = if working_dir.is_some() {
        working_dir.unwrap()
    } else {
        get_fur_directory()
    };

    let mut db_path = working_dir.clone();
    db_path.push(db_id);

    let db_info = DatabaseInfo::new(&db_name.as_ref().unwrap())?;

    Database::create_database(db_path.clone(), db_info)?;
    Database::get_database(db_path)
}

pub(crate) fn generate_table_info(
    table_info_generatable: models::CreateTableParams,
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
