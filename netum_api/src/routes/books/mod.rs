use sea_orm::{ ActiveValue::NotSet, ConnectionTrait, DbErr, Set };
use ::serde::{ Deserialize, Serialize };
use sea_orm::ActiveModelTrait;

use crate::database::book;

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

pub async fn create_with_isbn<C: ConnectionTrait>(
    isbn: String,
    location: Option<String>,
    lent_to: Option<String>,
    read: bool,
    db: &C
) -> Result<book::Model, DbErr> {
    let body = reqwest
        ::get(format!("https://www.googleapis.com/books/v1/volumes?q=isbn:{}", isbn)).await
        .map_err(|_| DbErr::Custom("Could not reach Google API".to_owned()))?
        .json::<BookAPI>().await
        .map_err(|_| DbErr::Custom("Could not map json from Google API".to_owned()))?;
    (book::ActiveModel {
        id: NotSet,
        title: Set(body.volume_info.title.clone()),
        sub_title: Set(Some(body.volume_info.sub_title.clone())),
        authors: Set(body.volume_info.authors.clone()),
        categories: Set(body.volume_info.categories.clone()),
        location: Set(location),
        read: Set(read),
        description: Set(Some(body.volume_info.description.clone())),
        lent_to: Set(lent_to),
    }).insert(db).await
}
