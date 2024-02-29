use actix_web::{get, web, HttpRequest, Responder};
use std::{error::Error, path::PathBuf};

mod utils;
use utils::{generate_table_info, get_db};

mod response;
use response::TableResponse;

mod params;
use params::TableParams;

mod request;
use request::QueryGeneratable;

#[get("/{db}/{tb}/data")]
pub(crate) async fn get_data_handler(
    path: web::Path<(String, String)>,
    req: HttpRequest,
    table_generatable: Option<web::Json<QueryGeneratable>>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (db, tb) = path.into_inner();
    let params = web::Query::<TableParams>::from_query(req.query_string()).unwrap();

    let working_dir = params.working_dir.as_ref().map(|wd| PathBuf::from(wd));
    let db = get_db(working_dir, &db, params.db_name.clone())?;

    let table_info = if table_generatable.is_some() {
        table_generatable
            .as_ref()
            .unwrap()
            .0
            .table_generatable
            .as_ref()
            .map(|table_generatable| generate_table_info(table_generatable.clone()).unwrap())
    } else {
        None
    };

    let mut tb = db.get_table(&tb, table_info)?;

    let data =
        if table_generatable.is_some() && table_generatable.as_ref().unwrap().0.query.is_some() {
            let table_generator = table_generatable.unwrap().0;
            let query = table_generator.query;
            let columns = tb.get_info()?.get_columns();
            let mut query_column = columns[0].clone();

            for column in columns {
                if query.clone().unwrap().column_id == column.get_id() {
                    query_column = column.clone();
                }
            }

            let index = tb.query(&query_column, &query.unwrap().value).await?;
            vec![tb.get_row(index.unwrap()).await?]
        } else {
            tb.get_all().await?
        };

    let res = TableResponse::new(data);

    Ok(web::Json(res))
}
