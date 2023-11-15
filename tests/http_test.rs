

use reqwest::StatusCode;
use serde_json::json;

#[tokio::test]
async fn login() {
    let login_url = "http://localhost:3000/restaurant/login";

    let params = json!({"phone":"12345678","password":"passwd"});

    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .build()
        .unwrap();

    let resp = client.post(login_url).json(&params).send().await.unwrap();
    if resp.status() == StatusCode::OK {
        let cookies = resp.cookies();
        for cookie in cookies {
            println!("Name: {}, Value: {}", cookie.name(), cookie.value());
        }
    } else {
        eprintln!("code {}", resp.status())
    }

    let res = client
        .get("http://localhost:3000/restaurant/goods/1")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    eprintln!("res:{}", res)
}
