use rocket::{ http::Status, post, response::status, serde::json::Json, State };
use sea_orm::DatabaseConnection;
use ::serde::{ Deserialize, Serialize };

use crate::{ database::{ book, location }, tables };

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BookAPI {
    #[serde(rename = "volumeInfo")]
    pub volume_info: VolumeInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VolumeInfo {
    pub title: String,
    #[serde(rename = "subtitle")]
    pub sub_title: String,
    pub authors: Vec<String>,
    pub description: String,
    pub categories: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWithISBNInput {
    isbn: String,
    location: String,
    lent_to: Option<String>,
    read: bool,
}
#[post("/create_with_isbn", format = "application/json", data = "<data>")]
pub async fn create_with_isbn(
    data: Json<CreateWithISBNInput>,
    db: &State<DatabaseConnection>
) -> Result<Json<book::Model>, status::Custom<Json<String>>> {
    let db = db.inner();
    let data = data.0;
    let book = tables::books::create_with_isbn(
        data.isbn,
        data.location,
        data.lent_to,
        data.read,
        db
    ).await;
    match book {
        Ok(book) => Ok(Json(book)),
        Err(e) =>
            Err(
                status::Custom(
                    Status::InternalServerError,
                    Json(format!("Error creating book: {:?}", e))
                )
            ),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationInput {
    pub name: String,
    pub color: String,
}

#[post("/create_location", format = "application/json", data = "<location>")]
pub async fn create_location(
    location: Json<LocationInput>,
    db: &State<DatabaseConnection>
) -> Result<Json<location::Model>, status::Custom<Json<String>>> {
    let db = db.inner();
    let location = location.0;
    let color = colors_transform::Rgb
        ::from_hex_str(location.color.as_str())
        .map_err(|_| status::Custom(Status::BadRequest, Json(format!("Invalid color input"))))?;
    let book = tables::books::create_location(location.name, color, db).await;
    match book {
        Ok(book) => Ok(Json(book)),
        Err(e) =>
            Err(
                status::Custom(
                    Status::InternalServerError,
                    Json(format!("Error creating location: {:?}", e))
                )
            ),
    }
}
