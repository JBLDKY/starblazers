use crate::claims::Claims;
use crate::multiplayer::actors::UserStateManager;
use crate::multiplayer::communication::message::CheckExistingConnection;
use crate::multiplayer::{ListLobbies, LobbyManager, PlayersInLobby, UserState, WsSession};
use crate::types::{LoginDetails, LoginMethod, Player, PublicUserRecord, User};
use crate::{database::db::ArcDb, index::INDEX_HTML};
use actix::Addr;
use actix_web::http::header::{ContentType, HeaderValue};
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde_json::json;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use uuid::Uuid;

/// GET /players/all - players_all - Returns all players
/// POST /players/create - players_create - Create a new player
/// POST /database/resettable - database_resettable - Drop the provided table
/// POST /auth/login - login - start authenticating the login request
/// GET /helloworld - helloworld - for sanity checks / testing warp things
/// GET /auth/verify_jwt
/// GET /players/player - return information about the player.
/// GET /game/lobbies - get a list of active lobbies
// [websocket] GET /lobby - the main (and currently only) websocket

/// Configure the server services
pub fn config_server(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(players_all)
        .service(users_all)
        .service(login)
        .service(sign_up)
        .service(lobby_websocket)
        .service(hello_world)
        .service(verify_jwt)
        .service(player_info)
        .service(list_lobbies)
        .service(players_in_lobby);
}

/// POST /auth/signup -> Returns TODO
#[post("/auth/signup")]
async fn sign_up(db: web::Data<ArcDb>, body: web::Bytes) -> Result<HttpResponse, actix_web::Error> {
    let user = serde_json::from_slice::<User>(&body)?;
    match db.create_user(&user).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({"message:": user.username}))),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e)),
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

/// GET /players/player endpoint.
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

fn jwt_check(req: &HttpRequest) -> Result<(), HttpResponse> {
    if let Some(header_value) = req.headers().get("Authorization") {
        if Claims::from_header_value(header_value).is_ok() {
            return Ok(());
        }
    }

    log::error!("Authorization header not found or invalid!");
    Err(HttpResponse::Unauthorized().finish())
}

/// GET /game/lobbies
///
/// Returns a list of currently available lobbies.
#[get("/game/lobbies")]
async fn list_lobbies(
    req: HttpRequest,
    lobby_manager: web::Data<Addr<LobbyManager>>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Err(e) = jwt_check(&req) {
        return Ok(e);
    }
    sleep(Duration::from_secs(2)).await; // Add a delay to simulate a real environment

    let lobbies = lobby_manager
        .send(ListLobbies)
        .await
        .expect("Failed to get a list of available lobbies.");

    json_with_status(&json!(lobbies), StatusCode::OK)
}

/// GET /game/lobbies
///
/// Returns a list of currently available lobbies.
#[get("/game/{lobby_name}/players")]
async fn players_in_lobby(
    req: HttpRequest,
    lobby_name: web::Path<String>,
    lobby_manager: web::Data<Addr<LobbyManager>>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Err(e) = jwt_check(&req) {
        return Ok(e);
    }

    let players = lobby_manager
        .send(PlayersInLobby {
            lobby_name: lobby_name.to_string(),
        })
        .await
        .expect("Failed to get a list of players in the lobby");

    json_with_status(&json!(players), StatusCode::OK)
}

/// Constructs a JSON response with a specific status code.
#[inline(always)]
fn json_with_status(
    json: &serde_json::Value,
    status: StatusCode,
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::build(status).json(json))
}

// [websocket] GET /lobby
#[get("/lobby")]
async fn lobby_websocket(
    req: HttpRequest,
    stream: web::Payload,
    lm: actix_web::web::Data<Addr<LobbyManager>>,
    usm: actix_web::web::Data<Addr<UserStateManager>>,
) -> Result<HttpResponse, actix_web::Error> {
    let header_value: Option<&HeaderValue> = req.headers().get("cookie");

    let claims;

    match header_value {
        Some(header_value) => match Claims::from_header_value(header_value) {
            Ok(v) => claims = v,
            Err(e) => {
                log::error!("Error validating jwt: {}", e);
                return Ok(HttpResponse::Unauthorized().finish());
            }
        },
        None => {
            log::error!("Authorization header not found!");
            return Ok(HttpResponse::Unauthorized().finish());
        }
    }

    let user_id = match claims.uuid() {
        Ok(v) => v,
        Err(_) => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let ws_connection_exists = usm.send(CheckExistingConnection { user_id }).await.ok();
    if ws_connection_exists.unwrap_or(false) {
        log::debug!("Websocket connection for {} already exists", user_id);
        return Ok(HttpResponse::Ok().finish());
    }
    ws::start(
        WsSession {
            connection_id: Uuid::new_v4(),
            user_id,
            user_state: UserState::Authenticated { player_id: user_id },
            hb: Instant::now(),
            lobby_manager_addr: lm.get_ref().clone(),
            user_state_manager_addr: usm.get_ref().clone(),
        },
        &req,
        stream,
    )
}
