use std::fmt::LowerHex;

use log::warn;
use sea_orm::{ ActiveValue::NotSet, ConnectionTrait, DbErr, Set };
use ::serde::{ Deserialize, Serialize };
use sea_orm::ActiveModelTrait;

use crate::database::{ book, location };

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BookAPIResponse {
    pub items: Vec<Items>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Items {
    #[serde(rename = "volumeInfo")]
    pub volume_info: VolumeInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VolumeInfo {
    pub title: String,
    #[serde(rename = "subtitle")]
    pub sub_title: Option<String>,
    pub authors: Vec<String>,
    pub description: String,
    pub categories: Vec<String>,
}

pub async fn create_with_isbn<C: ConnectionTrait>(
    isbn: String,
    location: String,
    lent_to: Option<String>,
    read: bool,
    db: &C
) -> Result<book::Model, DbErr> {
    let body = reqwest
        ::get(format!("https://www.googleapis.com/books/v1/volumes?q=isbn:{}", isbn)).await
        .map_err(|e| DbErr::Custom(format!("Could not reach Google API: {:?}", e)))?
        .json::<BookAPIResponse>().await
        .map_err(|_| DbErr::Custom("Could not map json from Google API".to_owned()))?.items;
    warn!("Body: {:?}", body);
    match body.first() {
        Some(body) =>
            (book::ActiveModel {
                id: NotSet,
                title: Set(body.volume_info.title.clone()),
                sub_title: Set(body.volume_info.sub_title.clone()),
                authors: Set(body.volume_info.authors.clone()),
                categories: Set(body.volume_info.categories.clone()),
                location: Set(location),
                read: Set(read),
                description: Set(Some(body.volume_info.description.clone())),
                lent_to: Set(lent_to),
                ..Default::default()
            }).insert(db).await,
        None => Err(DbErr::Custom("Could not find item".to_owned())),
    }
}

pub async fn create_location<C: ConnectionTrait>(
    location: String,
    color: colors_transform::Rgb,
    db: &C
) -> Result<location::Model, DbErr> {
    let location = location::ActiveModel {
        location: Set(location),
        color: Set(color.to_css_hex_string()),
    };
    location.insert(db).await
}
