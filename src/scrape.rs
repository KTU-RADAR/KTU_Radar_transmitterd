use axum::{response::IntoResponse, Json};
use scraper::{Html, Selector};

use crate::{sqlite_connection, HocaDuyuru, ScDuyuru};

async fn scrape_data(url: &str) -> Json<Vec<ScDuyuru>> {
    let mut duyuru = ScDuyuru {
        topic: "".to_owned(),
        author: "".to_owned(),
        date: "".to_owned(),
        link: "".to_owned(),
    };

    let ktu_duyuru = reqwest::get(url)
        .await
        .expect("ktu genel duyuru baglanamadi")
        .text()
        .await
        .expect("ktu genel duyuru body alinamadi");

    let parsed_html = Html::parse_fragment(&ktu_duyuru);
    let mut index = 1;

    let mut duyuru_json = vec![];

    while index < 19 {
        let topic_selector = "div.carousel-item:nth-child(1) > div:nth-child(1) > a:nth-child("
            .to_owned()
            + index.to_string().as_str()
            + ") > div:nth-child(1) > div:nth-child(2) > div:nth-child(1)";

        let parsed_topic_selector =
            Selector::parse(&topic_selector).expect("cant parse topic selector");

        for element in parsed_html.select(&parsed_topic_selector) {
            duyuru.topic = element.inner_html().trim_end().to_owned();
        }

        let author_selector = "div.carousel-item:nth-child(1) > div:nth-child(1) > a:nth-child("
            .to_owned()
            + index.to_string().as_str()
            + ") > div:nth-child(1) > div:nth-child(2) > p:nth-child(2)";

        let parsed_author_selector =
            Selector::parse(&author_selector).expect("cant parse author selector");

        for element in parsed_html.select(&parsed_author_selector) {
            duyuru.author = element.inner_html().trim_end().to_owned();
        }

        let date_selector = "div.carousel-item:nth-child(1) > div:nth-child(1) > a:nth-child("
            .to_owned()
            + index.to_string().as_str()
            + ") > div:nth-child(1) > div:nth-child(1) > h3:nth-child(1)";

        let parsed_dated_selector =
            Selector::parse(&date_selector).expect("cant parse date (day) selector");

        let mut day = "".to_owned();

        for element in parsed_html.select(&parsed_dated_selector) {
            day = element.inner_html().trim_end().to_owned();
        }

        let mut month = "".to_owned();

        let selector_base = "div.carousel-item:nth-child(1) > div:nth-child(1) > a:nth-child("
            .to_owned()
            + index.to_string().as_str()
            + ") > div:nth-child(1) > div:nth-child(1) > p:nth-child(2)";

        let parsed_datem_selector =
            Selector::parse(&selector_base).expect("cant parse date (month) selector");

        for element in parsed_html.select(&parsed_datem_selector) {
            month = element.inner_html().trim_end().to_owned();
        }

        duyuru.date = day + " " + month.as_str();

        let link_selector = "div.carousel-item:nth-child(1) > div:nth-child(1) > a:nth-child("
            .to_owned()
            + index.to_string().as_str()
            + ")";
        let parsed_link_selector =
            Selector::parse(&link_selector).expect("cant parse link selector");

        for element in parsed_html.select(&parsed_link_selector) {
            duyuru.link = "https://www.ktu.edu.tr".to_string()
                + element
                    .value()
                    .attr("href")
                    .expect("can't get href value of link")
                    .to_string()
                    .as_str();
        }

        duyuru_json.push(duyuru.clone());
        index += 1
    }
    Json(duyuru_json)
}

pub async fn ktu_duyuru() -> impl IntoResponse {
    scrape_data("https://www.ktu.edu.tr/tr/duyurular").await
}

pub async fn ktu_pc_duyuru() -> impl IntoResponse {
    scrape_data("https://www.ktu.edu.tr/bilgisayar/duyurular").await
}

pub async fn trigger_db() -> impl IntoResponse {
    let conn = sqlite_connection();
    let mut stmt = conn.prepare("SELECT * FROM duyurular").unwrap();
    let duyurular = stmt
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

    for duyuru in duyurular {
        println!("Found duyuru {:?}", duyuru.unwrap());
    }
}
