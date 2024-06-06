use crate::claims::{Claims, TokenError};
use crate::types::{LoginDetails, LoginMethod, Player, PublicUserRecord, User};
use crate::{
    database::db::ArcDb,
    database::queries::Table,
    websocket::{user_connected, Users, INDEX_HTML},
};
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use serde_json::json;
use warp::{http::header::HeaderValue, http::StatusCode, Filter, Reply};
/*
/// Generically parse a json body into a struct
fn json_body<T: Serialize + for<'a> Deserialize<'a> + Send + Sync>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

/// Parses the authorization header into Claims
fn with_decoded_jwt(header_value: HeaderValue) -> Result<Claims, TokenError> {
    Claims::from_header_value(header_value)
}
*/
/// TODO update
/// GET /players/all - players_all - Returns all players
/// POST /players/create - players_create - Create a new player
/// POST /database/resettable - database_resettable - Drop the provided table
/// POST /auth/login - login - start authenticating the login request
/// GET /helloworld - helloworld - for sanity checks / testing warp things
/// GET /auth/verify_jwt

/// Configure the server services
pub fn config_server(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        //.service(chat)
        .service(players_all)
        .service(users_all)
        .service(database_resettable)
        .service(login)
        .service(sign_up)
        .service(hello_world)
        .service(verify_jwt);
    //.service(player_info);
}

// Utility to pass the database pool into Warp filters
fn with_db(db: ArcDb) -> impl Filter<Extract = (ArcDb,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

/// POST /auth/signup -> Returns TODO
#[post("/auth/signup")]
async fn sign_up(
    db: web::Data<ArcDb>,
    user: web::Json<User>,
) -> Result<HttpResponse, actix_web::Error> {
    match db.create_user(&user).await {
        Ok(_) => Ok(HttpResponse::Ok().json(&json!({"message:": user.username}))),
        Err(e) => Err(actix_web::error::ErrorImATeapot(e)),
    }
}

/// GET /helloworld
#[get("/helloworld")]
async fn hello_world() -> String {
    format!("{:#?}", "Hi there!".to_owned())
}

#[get("/")]
async fn index() -> HttpResponse {
    //warp::path::end().map(|| warp::reply::html(INDEX_HTML))
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("X-Hdr", "sample"))
        .body(INDEX_HTML)
}

/// Returns all players from the Players table
#[get("/players/all")]
async fn players_all(db: web::Data<ArcDb>) -> Result<HttpResponse, actix_web::Error> {
    let result: Result<Vec<Player>, sqlx::Error> =
        sqlx::query_as!(Player, "SELECT * FROM players",)
            .fetch_all(&db.pool)
            .await;

    match result {
        Ok(players) => Ok(HttpResponse::Ok().json(players)),
        Err(e) => Err(actix_web::error::ErrorImATeapot(e)),
    }
}

/// Returns all users from the Users table
#[get("/users/all")]
async fn users_all(db: web::Data<ArcDb>) -> Result<HttpResponse, actix_web::Error> {
    let result: Result<Vec<User>, sqlx::Error> = sqlx::query_as!(User, "SELECT * FROM users",)
        .fetch_all(&db.pool)
        .await;

    match result {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(e) => Err(actix_web::error::ErrorImATeapot(e)),
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
#[post("/database/resettable")]
async fn database_resettable(
    db: web::Data<ArcDb>,
    body: actix_web::web::Json<Table>,
) -> Result<HttpResponse, actix_web::Error> {
    match db.reset_table(&body).await {
        Ok(v) => Ok(HttpResponse::Ok().json(&json!({"deleted_records": &v.rows_affected()}))),
        Err(e) => Err(actix_web::error::ErrorImATeapot(e)),
    }
}

/// POST /auth/login -> Try to login
#[post("/auth/login")]
async fn login(
    db: web::Data<ArcDb>,
    login_details: actix_web::web::Json<LoginDetails>,
) -> Result<HttpResponse, actix_web::Error> {
    match db.check_login_details(&login_details).await {
        Ok(jwt) => {
            log::info!("Logged in!");

            Ok(HttpResponse::Ok()
                .insert_header(("Authorization", "Bearer".to_owned() + &jwt))
                .finish())
        }
        Err(e) => {
            log::error!("Error during login: {}", e);
            Err(actix_web::error::ErrorImATeapot(e))
        }
    }
}

/// GET /auth/verify_jwt
#[get("/auth/verify_jwt")]
async fn verify_jwt(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let header_value: Option<&HeaderValue> = req.headers().get("authorization");

    match header_value {
        Some(header_value) => {
            match Claims::from_header_value(header_value.clone()) {
                Ok(_) => {
                    // TODO: Maybe add the succesful login to the database?
                    Ok(HttpResponse::Ok().body("JWT Valid"))
                }
                Err(e) => {
                    log::error!("Error validating jwt: {}", e);

                    Ok(HttpResponse::Unauthorized().finish())
                }
            }
        }
        None => {
            log::error!("Authorization header not found!");
            Ok(HttpResponse::Unauthorized().finish())
        }
    }
}

/// Creates the warp filter for the GET /players/player endpoint.
///
/// This filter extracts the authorization header, decodes it into claims, and retrieves player
/// information based on those claims from the database.
fn player_info(
    db: ArcDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("players" / "player")
        .and(warp::get())
        .and(warp::header::value("authorization"))
        .map(with_decoded_jwt)
        .and(with_db(db))
        .and_then(get_player_info)
}

/// Retrieves player information based on the provided JWT in the header.
///
/// Returns a 401 status code if the JWT is invalid or expired.
/// Returns a 404 status code if the user cannot be found in the database.
async fn get_player_info(
    claims: Result<Claims, TokenError>,
    db: ArcDb,
) -> Result<impl Reply, warp::Rejection> {
    // Token is expired or otherwise invalid - 401
    let (email, uuid) = match claims {
        Ok(claims) => (claims.sub, claims.uuid),
        Err(e) => {
            log::info!("Invalid JWT attempted to access user information: {}", e);
            return json_with_status(&json!({"error": "Unauthorized"}), StatusCode::UNAUTHORIZED);
        }
    };

    let query_result = db
        .get_details_by_login_method(&LoginMethod::Email(email))
        .await;

    match query_result {
        Ok(user_record) => {
            // Happy path: Found the UserRecord and removed the password.
            let public_user_record: PublicUserRecord = user_record.into();
            json_with_status(&json!(public_user_record), StatusCode::OK)
        }
        Err(_) => {
            // Could not find a user for this email in the database. Should not happen unless the
            // token contains a faked email.
            log::info!("Could not find user with UUID: {}", uuid);
            json_with_status(&json!({"error": "User not found"}), StatusCode::NOT_FOUND)
        }
    }
}

/// Constructs a JSON response with a specific status code.
#[inline(always)]
fn json_with_status(
    json: &serde_json::Value,
    status: StatusCode,
) -> Result<warp::reply::WithStatus<warp::reply::Json>, warp::Rejection> {
    Ok(warp::reply::with_status(warp::reply::json(json), status))
}
