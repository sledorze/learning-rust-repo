use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/hello2/toto").await?.print().await?;

    hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_get("/hello2/toto").await?.print().await?;

    hc.do_post("/api/tickets", json!({"title": "toto"}))
        .await?
        .print()
        .await?;

    hc.do_get("/api/tickets").await?.print().await?;

    hc.do_delete("/api/tickets/0").await?.print().await?;

    Ok(())
}
