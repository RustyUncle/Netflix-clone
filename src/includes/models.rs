use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Createuser {

    pub fname: String,
    pub lname: String,
    pub uname: String,
    pub email: String,
    pub email2: String,
    pub pass: String,
    pub pass2: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Loginuser {

    pub email: String,
    pub pass: String,
}

#[derive(Serialize,Deserialize, Clone, PartialEq, FromRow)]
pub struct Updatepassword {
    pub oldpass: String,
    pub newpass: String,
    pub newpass2: String,
}

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub fname: String,
    pub lname: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Entity {
    id: i32,
    name: String,
    thumbnail: String,
    preview: String,
    categoryid: i32,
}


#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct Categories {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct Videos {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub filepath: String,
    pub ismovie: i16,
    pub views: i32,
    pub duration: String,
    pub season: i32,
    pub episode: i32,
    pub entityid: i32,

}

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct Progress {
    pub videoid: i32,
    pub username: String
    
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Videoprogress{
    pub username: String,
    pub videoid: i32,
    pub progress: f32, 
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Search {
    pub term: String,
    pub username: String,
}