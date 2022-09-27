use tokio_postgres::{NoTls};
use deadpool_postgres::{Config, ManagerConfig, Client, Pool, RecyclingMethod, Runtime};
use actix_web::{get, web, App, HttpServer, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::{FromTokioPostgresRow};
use tokio_pg_mapper_derive::{PostgresMapper};

// TODO: error handling
// TODO: module organization

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // db setup
    let mut cfg = Config::new();
    cfg.dbname = Some("fs".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("postgres".to_string());
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    // connection pool
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    // start server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(test_read_db)
    })
    .bind("127.0.0.1:3000")
    .unwrap()
    .run();

    server.await
}

// A SIMPLE GET METHOD
#[get("/organizations")]
async fn test_read_db(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.unwrap();
    let organizations: Vec<Organization> = client.query("SELECT * FROM organizations", &[]).await.unwrap().iter()
        .map(|row| Organization::from_row_ref(row).unwrap())
        .collect::<Vec<Organization>>();

    Ok(HttpResponse::Ok().json(organizations))
}

// BASIC MODEL
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "organizations")]
struct Organization {
    id: i32,
    name: String
}
