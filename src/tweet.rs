use actix_web::dev::Response;
use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::APPLICATION_JSON;
use crate::like::Like;
use crate::response::Response;
pub type Tweets = Response<Tweet>;
#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    created_at: String,
    id: u64,
    id_str: String,
    text: String,
    source: String,
    truncated: bool,
    in_reply_to_status_id: Option<u64>,
    in_reply_to_status_id_str: Option<String>,
    in_reply_to_user_id: Option<u64>,
    in_reply_to_user_id_str: Option<String>,
    in_reply_to_screen_name: Option<String>,
    user: User,
    geo: Option<Geo>,
    coordinates: Option<Coordinates>,
    place: Option<Place>,
    contributors: Option<Vec<Contributor>>,
    is_quote_status: bool,
    retweet_count: u64,
    favorite_count: u64,
    favorited: bool,
    retweeted: bool,
    possibly_sensitive: Option<bool>,
    lang: String,
}