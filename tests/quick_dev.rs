#![allow(unused)] // For beginning only.

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    // The ? means: If this function call returns an Err, return it immediately from the current function. If it’s Ok(value), extract the value and keep going.
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // How To Test A Route: Follow the patterns below to add a test route. Then run `make watch-tests`.
    // NOTES:
    // * Each await waits for an asynchronous operation to finish. Each ? checks for errors and returns early if something failed.
    // * So this line means: Make an HTTP GET request to /query-params-route. Wait for the response. If it succeeded, print it; otherwise, return the error. Wait for printing to finish. If that also succeeded, continue; otherwise, return the error.”
    // hc.do_get("/query-params-route?name=Sam").await?.print().await?;
    hc.do_get("/path-params-route/Sam").await?.print().await?;

    hc.do_get("/src/main.rs").await?.print().await?;

    // () is called the unit type — it’s like “nothing” or “no meaningful value.”
    // So this means that the operation succeeded, and there’s no meaningful value to return.
    return Ok(());
}
