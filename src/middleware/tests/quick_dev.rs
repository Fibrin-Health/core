#![allow(unused)]

use anyhow::Result;
use serde_json::json;



#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello2/Akan").await?.print().await?;

    // hc.do_get("/src/main.rs").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        })
    );

    req_login.await?.print().await?;


    let req_create_ehr = hc.do_post(
        "/api/ehrs",
        json!({
            "title": "Ehr 1"
        })
    );

    req_create_ehr.await?.print().await?;

    // hc.do_delete("/api/ehrs/2").await?.print().await?;

    hc.do_get("/api/ehrs").await?.print().await?;

    Ok(())
}