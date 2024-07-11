use rocket::{get, post, put, delete, Rocket, State};
use rocket_contribe::json::json;
use serde::{Serialize, Deserialize};

use crate::domain::Post;
use crate::infrastructure::{establish_connection, run_migrations};

#[derive(Serialize, Deserialize)]
struct Auth {
    username: String,
    password: String,
}

#[post("/login", data = "<auth>")]
fn login(auth: Json<Auth>) -> Json<String>{
    /////
    Json("jwt_token".to_String())
}
#[get("/posts")]
fn get_posts() -> Json<Vec<Post>> {
    use crate::schema::posts::dsl::*;
    let connection = establish_connection();
    let posts = posts.load::<Post>(&connection).unwrap();
    Json(posts)
}

#[post("/posts", data = "<post>")]
fn create_post(post: Json<Post>) -> Json<Post> {
    use crate::schema::posts::dsl::*;
    let connection = establish_connection();
    let new_post = post.into_inner();
    diesel::insert_into(posts)
        .values(&new_post)
        .execute(&connection)
        .unwrap();
    Json(new_post)
}

#[put("/posts/<id>", data = "<post>")]
fn update_post(id: i32, post: Json<Post>) -> Json<Post> {
    use crate::schema::posts::dsl::*;
    let connection = establish_connection();
    let updated_post = post.into_inner();
    diesel::update(posts.find(id))
        .set(&updated_post)
        .execute(&connection)
        .unwrap();
    Json(updated_post)
}

#[delete("/posts/<id>")]
fn delete_post(id: i32) -> Json<String> {
    use crate::schema::posts::dsl::*;
    let connection = establish_connection();
    diesel::delete(posts.find(id))
        .execute(&connection)
        .unwrap();
    Json("Post deleted successfully".to_string())
}

fn main() {
    run_migrations();
    let rocket = Rocket::build()
        .mount("/", routes![login, get_posts, create_post, update_post, delete_post])
        .manage(establish_connection());
    rocket.launch().unwrap();
}