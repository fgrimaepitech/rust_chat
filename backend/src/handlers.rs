use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
}

#[derive(Clone)]
pub struct AppState {
    pub redis_client: redis::Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub channel: String,
    pub sender: String,
    pub content: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct NewMessage {
    pub channel: String,
    pub sender: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct GetMessagesQuery {
    pub channel: String,
    pub limit: Option<usize>,
}

#[post("/messages")]
pub async fn post_message(
    data: web::Data<AppState>,
    body: web::Json<NewMessage>,
) -> impl Responder {
    let msg = body.into_inner();
    let chat_msg = ChatMessage {
        id: Uuid::new_v4().to_string(),
        channel: msg.channel.clone(),
        sender: msg.sender,
        content: msg.content,
        timestamp: Utc::now(),
    };

    let serialized = match serde_json::to_string(&chat_msg) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Échec de sérialisation : {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let list_key = format!("chat:{}", chat_msg.channel);
    let mut conn = match data.redis_client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => {
            log::error!("Erreur connexion Redis : {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if let Err(e) = conn.lpush::<_, _, ()>(&list_key, &serialized).await {
        log::error!("LPUSH failed : {}", e);
    }
    let _: () = conn.ltrim(&list_key, 0, 99).await.unwrap_or(());

    if let Err(e) = conn.publish::<_, _, ()>(&chat_msg.channel, &serialized).await {
        log::error!("PUBLISH failed : {}", e);
    }

    HttpResponse::Created().json(&chat_msg)
}

#[get("/messages")]
pub async fn get_messages(
    data: web::Data<AppState>,
    params: web::Query<GetMessagesQuery>,
) -> impl Responder {
    let channel = &params.channel;
    let limit = params.limit.unwrap_or(50);
    let list_key = format!("chat:{}", channel);

    let mut conn = match data.redis_client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => {
            log::error!("Erreur connexion Redis : {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let raw: Vec<String> = match conn.lrange(&list_key, 0, (limit - 1) as isize).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("LRANGE failed : {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let mut msgs = Vec::with_capacity(raw.len());
    for entry in raw {
        match serde_json::from_str::<ChatMessage>(&entry) {
            Ok(m) => msgs.push(m),
            Err(e) => log::error!("Parse message JSON failed : {}", e),
        }
    }

    HttpResponse::Ok().json(msgs)
}

#[post("/channels")]
pub async fn create_channel(
    data: web::Data<AppState>,
    body: web::Json<CreateChannelRequest>,
) -> impl Responder {
    let channel = Channel {
        id: Uuid::new_v4().to_string(),
        name: body.name.clone(),
        created_at: Utc::now(),
    };

    let serialized = match serde_json::to_string(&channel) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Channel serialization failed: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let mut conn = match data.redis_client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => {
            log::error!("Redis connection error: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if let Err(e) = conn.hset::<_, _, _, ()>("channels", &channel.id, &serialized).await {
        log::error!("Failed to store channel: {}", e);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Created().json(&channel)
}

#[get("/channels")]
pub async fn list_channels(data: web::Data<AppState>) -> impl Responder {
    let mut conn = match data.redis_client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => {
            log::error!("Redis connection error: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let channels: Vec<String> = match conn.hvals("channels").await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Failed to get channels: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let mut result = Vec::with_capacity(channels.len());
    for channel_str in channels {
        if let Ok(channel) = serde_json::from_str::<Channel>(&channel_str) {
            result.push(channel);
        }
    }

    HttpResponse::Ok().json(result)
}

#[post("/channels/{channel_id}/join")]
pub async fn join_channel(
    data: web::Data<AppState>,
    channel_id: web::Path<String>,
) -> impl Responder {
    let mut conn = match data.redis_client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => {
            log::error!("Redis connection error: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let exists: bool = match conn.hexists("channels", &*channel_id).await {
        Ok(exists) => exists,
        Err(e) => {
            log::error!("Failed to check channel existence: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if !exists {
        return HttpResponse::NotFound().finish();
    }

    HttpResponse::Ok().json(serde_json::json!({
        "status": "joined",
        "channel_id": channel_id.to_string()
    }))
}
