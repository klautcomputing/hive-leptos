use crate::websockets::{connection::WsConnection, lobby::Lobby};
use actix::Addr;
use actix_identity::Identity;
use actix_web::{get, web::Data, web::Payload, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use db_lib::{models::user::User, DbPool};
use uuid::Uuid;

#[get("/ws/")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    // group_id: Path<String>,
    srv: Data<Addr<Lobby>>,
    pool: Data<DbPool>,
    identity: Option<Identity>,
) -> Result<HttpResponse, Error> {
    if let Some(id) = identity {
        if let Ok(id_string) = id.id() {
            if let Ok(uuid) = Uuid::parse_str(&id_string) {
                if let Ok(user) = User::find_by_uuid(&uuid, &pool).await {
                    println!("Welcome {}!", user.username);
                    let ws = WsConnection::new(
                        Some(uuid),
                        user.username,
                        srv.get_ref().clone(),
                        pool.get_ref().clone(),
                    );
                    let resp = ws::start(ws, &req, stream)?;
                    return Ok(resp);
                }
            }
        }
    };

    println!("Welcome Anonymous!");
    let ws = WsConnection::new(
        None,
        String::from("Anonymous"),
        srv.get_ref().clone(),
        pool.get_ref().clone(),
    );

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}
