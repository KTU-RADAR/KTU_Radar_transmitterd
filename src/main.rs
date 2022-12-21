pub mod scrape;

use std::collections::HashMap;

use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::post;
use axum::Json;
use axum::Router;
use rusqlite::Connection;
use scrape::ktu_duyuru;
use scrape::ktu_pc_duyuru;
use scrape::trigger_db;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ScDuyuru {
    topic: String,
    author: String,
    date: String,
    link: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct HocaDuyuru {
    hoca: String,
    ders: String,
    konu: String,
    metin: String,
    tarih: String,
}

fn sqlite_connection() -> Connection {
    Connection::open("lol.db3").unwrap()
}

#[tokio::main]
async fn main() {
    if std::path::Path::new("lol.db3").exists() {
        println!("db exists")
    } else {
        let conn = sqlite_connection();
        conn.execute(
            "CREATE TABLE duyurular (
                hoca TEXT,
                ders TEXT,
                konu TEXT,
                metin TEXT,
                tarih TEXT
            )",
            (),
        )
        .unwrap();
    }

    ktu_duyuru().await;
    ktu_pc_duyuru().await;

    // build our application with a single route
    let app = Router::new()
        .route("/trigger", get(trigger_db))
        .route("/ktuduyuru", get(ktu_duyuru))
        .route("/", get(ktu_pc_duyuru))
        .route("/hocaduyuru", get(hoca_duyuru))
        .route("/duyuruekle", post(create_duyuru));
    // `POST /users` goes to `create_user`
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hoca_duyuru() -> impl IntoResponse {
    let conn = sqlite_connection();
    let mut stmt = conn.prepare("SELECT * FROM duyurular").unwrap();
    let person_iter = stmt
        .query_map([], |row| {
            Ok(HocaDuyuru {
                hoca: row.get(0)?,
                ders: row.get(1)?,
                konu: row.get(2)?,
                metin: row.get(3)?,
                tarih: row.get(4)?,
            })
        })
        .unwrap();

    let mut jsoned_hoca_duyuru: Vec<_> = vec![];
    for person in person_iter {
        let cloned_person = person.unwrap().clone();
        println!("Found person {:?}", cloned_person.clone());
        jsoned_hoca_duyuru.push(cloned_person)
    }
    Json(jsoned_hoca_duyuru)
}

async fn create_duyuru(Json(payload): Json<HocaDuyuru>) -> impl IntoResponse {
    println!("{:?}", payload);
    let clonedlol = payload.clone();
    let a = payload.hoca;
    let b = payload.ders;
    let c = payload.konu;
    let d = payload.metin;
    let e = payload.tarih;

    let baslik = c.clone() + ": " + &d;

    let client = fcm::Client::new();
    let mut map = HashMap::new();
    map.insert("key1", &b);

    let topic = "/topics/".to_string() + &b;

    let mut notification_builder = fcm::NotificationBuilder::new();
    notification_builder.title(&baslik);
    notification_builder.body(&d);

    let notification = notification_builder.finalize();
    let mut message_builder = fcm::MessageBuilder::new("", &topic);
    message_builder.data(&map).unwrap();
    message_builder.notification(notification);

    let response = client.send(message_builder.finalize()).await.unwrap();
    println!("Sent: {:?}", response);

    let conn = sqlite_connection();
    conn.execute(
        "INSERT INTO duyurular (hoca, ders, konu, metin, tarih) VALUES (?1,?2,?3,?4,?5)",
        (a, b, c, d, e),
    )
    .unwrap();
    Json(clonedlol)
}
