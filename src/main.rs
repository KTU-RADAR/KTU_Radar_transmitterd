use axum::response::IntoResponse;
use axum::routing::get;
use axum::Json;
use axum::Router;
use scraper::{Html, Selector};
use serde::Deserialize;
use serde::Serialize;

#[tokio::main]
async fn main() {
    ktu_duyuru().await;
    ktu_pc_duyuru().await;

    // build our application with a single route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(ktu_pc_duyuru));
    // `POST /users` goes to `create_user`
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Duyuru {
    topic: String,
    author: String,
    date: String,
}

async fn ktu_duyuru() -> impl IntoResponse {
    let mut duyuru = Duyuru {
        topic: "bruh".to_owned(),
        author: "bruh2".to_owned(),
        date: "bruh3".to_owned(),
    };

    let ktu_duyuru = reqwest::get("https://www.ktu.edu.tr/tr/duyurular")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    //println!("{}", ktu_duyuru);

    let fragment = Html::parse_fragment(&ktu_duyuru);
    let mut indis = 1;

    while indis < 19 {
        let selector_base = "div.carousel-item:nth-child(1) > div:nth-child(1) > div:nth-child("
            .to_owned()
            + indis.to_string().as_str()
            + ") > div:nth-child(1) > div:nth-child(2) > div:nth-child(1)";
        let selector = Selector::parse(&selector_base).unwrap();

        for element in fragment.select(&selector) {
            //println!("{}", element.inner_html().trim_end());
            duyuru.topic = element.inner_html().trim_end().to_owned();
        }

        let selector_base = "div.carousel-item:nth-child(1) > div:nth-child(1) > div:nth-child("
            .to_owned()
            + indis.to_string().as_str()
            + ") > div:nth-child(1) > div:nth-child(2) > p:nth-child(2)";
        let selector = Selector::parse(&selector_base).unwrap();

        for element in fragment.select(&selector) {
            //println!("{}", element.inner_html().trim_end());
            duyuru.author = element.inner_html().trim_end().to_owned();
        }
        //println!("{:?}", duyuru);

        let selector_base = "div.carousel-item:nth-child(1) > div:nth-child(1) > div:nth-child("
            .to_owned()
            + indis.to_string().as_str()
            + ") > div:nth-child(1) > div:nth-child(1) > h3:nth-child(1)";
        let selector = Selector::parse(&selector_base).unwrap();

        let mut temp_date = "bruh".to_owned();

        for element in fragment.select(&selector) {
            //println!("{}", element.inner_html().trim_end());
            temp_date = element.inner_html().trim_end().to_owned();
        }

        let mut temp_date2 = "bruh2".to_owned();

        let selector_base = "div.carousel-item:nth-child(1) > div:nth-child(1) > div:nth-child("
            .to_owned()
            + indis.to_string().as_str()
            + ") > div:nth-child(1) > div:nth-child(1) > p:nth-child(2)";
        let selector = Selector::parse(&selector_base).unwrap();

        for element in fragment.select(&selector) {
            //println!("{}", element.inner_html().trim_end());
            temp_date2 = element.inner_html().trim_end().to_owned();
        }

        duyuru.date = temp_date + " " + temp_date2.as_str();

        let serialized = serde_json::to_string(&duyuru).unwrap();
        println!("serialized = {}", serialized);

        indis += 1
    }

    Json(duyuru)
    /*let mut indis = 1;

    //div.carousel-item:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(2) > p:nth-child(2)
    //div.carousel-item:nth-child(1) > div:nth-child(1) > div:nth-child(18) > div:nth-child(1) > div:nth-child(2) > p:nth-child(2)
    while indis < 19 {
        let selector_base = "div.carousel-item:nth-child(1) > div:nth-child(1) > div:nth-child("
            .to_owned()
            + indis.to_string().as_str()
            + ") > div:nth-child(1) > div:nth-child(2) > p:nth-child(2)";
        let selector = Selector::parse(&selector_base).unwrap();

        for element in fragment.select(&selector) {
            println!("{}", element.inner_html().trim_end());
        }
        indis += 1
    }*/
}

async fn ktu_pc_duyuru() -> impl IntoResponse {
    let mut duyuru = Duyuru {
        topic: "bruh".to_owned(),
        author: "bruh2".to_owned(),
        date: "bruh3".to_owned(),
    };

    let ktu_duyuru = reqwest::get("https://www.ktu.edu.tr/bilgisayar/duyurular")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    //println!("{}", ktu_duyuru);
    //div.carousel-item:nth-child(1) > div:nth-child(1) > a:nth-child(1) > div:nth-child(1) > div:nth-child(2) > div:nth-child(1)
    //div.carousel-item:nth-child(1) > div:nth-child(1) > a:nth-child(2) > div:nth-child(1) > div:nth-child(2) > div:nth-child(1)
    //div.carousel-item:nth-child(1) > div:nth-child(1) > a:nth-child(2) > div:nth-child(1) > div:nth-child(2) > p:nth-child(2)
    //div.carousel-item:nth-child(1) > div:nth-child(1) > a:nth-child(1) > div:nth-child(1) > div:nth-child(1) > h3:nth-child(1)
    //div.carousel-item:nth-child(1) > div:nth-child(1) > a:nth-child(2) > div:nth-child(1) > div:nth-child(1) > p:nth-child(2)
    let fragment = Html::parse_fragment(&ktu_duyuru);
    let mut indis = 1;

    let mut json_total = vec![];

    while indis < 19 {
        let selector_base = "div.carousel-item:nth-child(1) > div:nth-child(1) > a:nth-child("
            .to_owned()
            + indis.to_string().as_str()
            + ") > div:nth-child(1) > div:nth-child(2) > div:nth-child(1)";
        let selector = Selector::parse(&selector_base).unwrap();

        for element in fragment.select(&selector) {
            //println!("{}", element.inner_html().trim_end());
            duyuru.topic = element.inner_html().trim_end().to_owned();
        }

        let selector_base = "div.carousel-item:nth-child(1) > div:nth-child(1) > a:nth-child("
            .to_owned()
            + indis.to_string().as_str()
            + ") > div:nth-child(1) > div:nth-child(2) > p:nth-child(2)";
        let selector = Selector::parse(&selector_base).unwrap();

        for element in fragment.select(&selector) {
            //println!("{}", element.inner_html().trim_end());
            duyuru.author = element.inner_html().trim_end().to_owned();
        }

        let selector_base = "div.carousel-item:nth-child(1) > div:nth-child(1) > a:nth-child("
            .to_owned()
            + indis.to_string().as_str()
            + ") > div:nth-child(1) > div:nth-child(1) > h3:nth-child(1)";
        let selector = Selector::parse(&selector_base).unwrap();

        let mut temp_date = "bruh".to_owned();

        for element in fragment.select(&selector) {
            //println!("{}", element.inner_html().trim_end());
            temp_date = element.inner_html().trim_end().to_owned();
        }

        let mut temp_date2 = "bruh2".to_owned();

        let selector_base = "div.carousel-item:nth-child(1) > div:nth-child(1) > a:nth-child("
            .to_owned()
            + indis.to_string().as_str()
            + ") > div:nth-child(1) > div:nth-child(1) > p:nth-child(2)";
        let selector = Selector::parse(&selector_base).unwrap();

        for element in fragment.select(&selector) {
            //println!("{}", element.inner_html().trim_end());
            temp_date2 = element.inner_html().trim_end().to_owned();
        }

        duyuru.date = temp_date + " " + temp_date2.as_str();

        let serialized = serde_json::to_string(&duyuru).unwrap();
        println!("serialized = {}", serialized);

        json_total.push(duyuru.clone());
        indis += 1
    }
    Json(json_total)

    /*let mut indis = 1;

    //div.carousel-item:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(2) > p:nth-child(2)
    //div.carousel-item:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(2) > p:nth-child(2)
    while indis < 19 {
        let selector_base = "div.carousel-item:nth-child(1) > div:nth-child(1) > div:nth-child("
            .to_owned()
            + indis.to_string().as_str()
            + ") > div:nth-child(1) > div:nth-child(2) > p:nth-child(2)";
        let selector = Selector::parse(&selector_base).unwrap();

        for element in fragment.select(&selector) {
            //println!("{}", element.inner_html().trim_end());
        }
        indis += 1
    }*/
}
