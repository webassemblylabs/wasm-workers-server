use anyhow::Result;
use wasm_workers_rs::{
    handler,
    http::{self, Request, Response},
    Cache, Content,
};

#[handler(cache)]
fn handler(_req: Request<String>, cache: &mut Cache) -> Result<Response<Content>> {
    // Applied changes here to use the Response method. This requires changes
    // on signature and how it returns the data.
    let count = cache.get("counter");
    let count_num = match count {
        Some(count_str) => count_str.parse::<u32>().unwrap_or(0),
        None => 0,
    };

    let response = format!(
        "<!DOCTYPE html>
<body>
  <h1>Key / Value store in Rust</h1>
  <p>Counter: {}</p>
  <p>This page was generated by a Wasm modules built from Rust.</p>
</body>",
        count_num
    );

    cache.insert("counter".to_string(), (count_num + 1).to_string());

    Ok(http::Response::builder()
        .status(200)
        .header("x-generated-by", "wasm-workers-server")
        .body(response.into())?)
}
