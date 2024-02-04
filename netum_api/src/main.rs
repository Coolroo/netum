use std::env;

use log::info;
use migration::MigratorTrait;
use rocket::{ get, launch, routes };
use sea_orm::{ ConnectOptions, Database, DatabaseConnection, DbErr };
use serde::{ Deserialize, Serialize };

mod entities;

#[derive(Deserialize, Serialize)]
struct Error {
    message: String,
}

#[get("/")]
fn index() -> String {
    format!("Welcome to the API :)")
}

pub async fn get_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(
        ConnectOptions::new(env::var("DATABASE_URL").expect("Could not load database url"))
            .max_connections(5)
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Trace)
            .clone()
    ).await?;
    info!("Beginning migrations...");
    migration::Migrator::up(&db, None).await?;
    Ok(db)
}

#[launch]
async fn rocket() -> _ {
    let db = match get_db().await {
        Ok(db_connection) => db_connection,
        Err(e) => panic!("Failed to connect to database: {}", e.to_string()),
    };

    db.ping().await.expect("Failed to connect to database!");

    rocket::build().manage(db).mount("/", routes![index])
}
