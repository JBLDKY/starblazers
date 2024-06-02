use crate::cookie::Cookie;
use crate::types::{DatabaseError, LoginDetails, Player, User};
use crate::{
    database::db::ArcDb,
    database::queries::Table,
    websocket::{user_connected, Users, INDEX_HTML},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use warp::{Filter, Reply};

/// Generically parse a json body into a struct
fn json_body<T: Serialize + for<'a> Deserialize<'a> + Send + Sync>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

/// Returns all the current filters
///
/// GET /players/all - players_all - Returns all players
/// POST /players/create - players_create - Create a new player
/// POST /database/resettable - database_resettable - Drop the provided table
/// POST /auth/login - login - start authenticating the login request
/// GET /helloworld - helloworld - for sanity checks / testing warp things
///
pub fn all(
    db: ArcDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_origin("http://localhost:5173")
        .allow_credentials(true)
        .allow_headers(vec!["Content-Type", "*"])
        .allow_headers(vec!["Authorization", "*"])
        .allow_methods(vec!["GET", "POST", "PUT"]);

    index()
        .or(chat())
        .or(players_all(db.clone()))
        .or(users_all(db.clone()))
        .or(database_resettable(db.clone()))
        .or(login(db.clone()))
        .or(hello_world())
        .or(sign_up(db.clone()))
        .with(cors)
}

// Utility to pass the database pool into Warp filters
fn with_db(db: ArcDb) -> impl Filter<Extract = (ArcDb,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

/// POST /auth/signup -> Returns TODO
fn sign_up(
    db: ArcDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("auth" / "signup")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(register_user)
}

async fn register_user(user: User, db: ArcDb) -> Result<impl Reply, warp::Rejection> {
    match db.create_user(&user).await {
        Ok(_) => Ok(warp::reply::json(&json!({"message:": user.username}))),
        Err(e) => Err(warp::reject::custom(DatabaseError(e))),
    }
}

/// GET /helloworld  ->  Returns {"message": "Hello World"}
fn hello_world() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("helloworld")
        .and(warp::get())
        .and(warp::header::headers_cloned())
        .map(|h| {
            println!("{:#?}", h);
            "hello"
        })
}

/// GET / -> index html
fn index() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end().map(|| warp::reply::html(INDEX_HTML))
}

/// GET /players/all -> Returns all players
fn players_all(
    db: ArcDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("players" / "all")
        .and(warp::get())
        .and(with_db(db))
        .and_then(fetch_all_players)
}

/// Returns all players from the Players table
async fn fetch_all_players(db: ArcDb) -> Result<impl Reply, warp::Rejection> {
    let result: Result<Vec<Player>, sqlx::Error> =
        sqlx::query_as!(Player, "SELECT * FROM players",)
            .fetch_all(&db.pool)
            .await;

    match result {
        Ok(players) => Ok(warp::reply::json(&json!(players))),
        Err(e) => Err(warp::reject::custom(DatabaseError(e))),
    }
}

/// GET /users/all -> Returns all users
fn users_all(
    db: ArcDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("users" / "all")
        .and(warp::get())
        .and(with_db(db))
        .and_then(fetch_all_users)
}

/// Returns all users from the users table
async fn fetch_all_users(db: ArcDb) -> Result<impl Reply, warp::Rejection> {
    let result: Result<Vec<User>, sqlx::Error> = sqlx::query_as!(User, "SELECT * FROM users",)
        .fetch_all(&db.pool)
        .await;

    match result {
        Ok(users) => Ok(warp::reply::json(&json!(users))),
        Err(e) => Err(warp::reject::custom(DatabaseError(e))),
    }
}

/// GET /chat -> websocket upgrade
fn chat() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Keep track of all connected users, key is usize, value
    // is a websocket sender.
    let users = Users::default();
    // Turn our "state" into a new Filter...
    let users = warp::any().map(move || users.clone());

    warp::path("chat")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .and(users)
        .map(|ws: warp::ws::Ws, users| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| user_connected(socket, users))
        })
}

/// POST /database/resettable -> drop(?) a table
fn database_resettable(
    db: ArcDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("database" / "resettable")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(reset_table)
}

async fn reset_table(table: Table, db: ArcDb) -> Result<impl Reply, warp::Rejection> {
    match db.reset_table(&table).await {
        Ok(v) => Ok(warp::reply::json(
            &json!({"deleted_records": &v.rows_affected()}),
        )),
        Err(e) => Err(warp::reject::custom(DatabaseError(e))),
    }
}

/// POST /auth/login -> Try to login
fn login(db: ArcDb) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("auth" / "login")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handle_login)
}

async fn handle_login(
    login_details: LoginDetails,
    db: ArcDb,
) -> Result<impl Reply, warp::Rejection> {
    match db.check_login_details(&login_details).await {
        Ok(jwt) => {
            log::info!("Logged in!");

            let mut cookie = Cookie::new("jwt".to_string(), jwt.clone());

            cookie.set_max_age(3600).set_secure(false);
            Ok(warp::reply::with_header(
                warp::reply::json(&json!({ "message": "Logged in", "jwt" : jwt })),
                "Set-Cookie",
                cookie.to_string(),
            ))
        }
        Err(e) => {
            log::error!("Error during login: {}", e);
            Err(warp::reject::custom(e))
        }
    }
}
