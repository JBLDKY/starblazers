use crate::claims::Claims;
use crate::types::{LoginDetails, LoginMethod, Player, PublicUserRecord, User};
use crate::websocket::MyWebSocket;
use crate::{database::db::ArcDb, websocket::INDEX_HTML};
use actix_web::http::header::{ContentType, HeaderValue};
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde_json::json;

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
        .service(web::resource("/ws").route(web::get().to(echo_websocket)))
        .service(players_all)
        .service(users_all)
        .service(login)
        .service(sign_up)
        .service(hello_world)
        .service(verify_jwt)
        .service(player_info);
}

async fn echo_websocket(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    ws::start(MyWebSocket::new(), &req, stream)
}

/// POST /auth/signup -> Returns TODO
#[post("/auth/signup")]
async fn sign_up(db: web::Data<ArcDb>, body: web::Bytes) -> Result<HttpResponse, actix_web::Error> {
    let user = serde_json::from_slice::<User>(&body)?;
    match db.create_user(&user).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({"message:": user.username}))),
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

/// POST /auth/login -> Try to login
#[post("/auth/login")]
async fn login(db: web::Data<ArcDb>, body: web::Bytes) -> Result<HttpResponse, actix_web::Error> {
    let login_details = serde_json::from_slice::<LoginDetails>(&body)?;
    match db.check_login_details(&login_details).await {
        Ok(jwt) => {
            log::info!("Logged in!");

            Ok(HttpResponse::Ok()
                .insert_header(("Authorization", "Bearer ".to_owned() + &jwt))
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
            match Claims::from_header_value(header_value) {
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
#[get("/players/player")]
async fn player_info(
    req: HttpRequest,
    db: web::Data<ArcDb>,
) -> Result<HttpResponse, actix_web::Error> {
    let header_value: Option<&HeaderValue> = req.headers().get("authorization");

    // Token is not in the headers
    if header_value.is_none() {
        log::error!("Authorization header not found!");
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let claims = Claims::from_header_value(header_value.unwrap());

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

/// Retrieves player information based on the provided JWT in the header.
///
/// Returns a 401 status code if the JWT is invalid or expired.
/// Returns a 404 status code if the user cannot be found in the database.

/// Constructs a JSON response with a specific status code.
#[inline(always)]
fn json_with_status(
    json: &serde_json::Value,
    status: StatusCode,
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::build(status).json(json))
}
