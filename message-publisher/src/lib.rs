use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
    redis
};

/// A simple Spin HTTP component.
#[http_component]
fn message_publisher(req: Request) -> Result<Response> {
    if req.method() != http::Method::POST {
        return Ok(http::Response::builder()
            .status(405)
            .body(Some("Method not allowed".into()))?);
    }
    let adr = std::env::var("REDIS_ENDPOINT").unwrap();
    let chn = std::env::var("REDIS_CHANNEL").unwrap();
    let p = req.body().as_ref().unwrap();
    return match redis::publish(&adr, &chn,&p ) {
        Ok(_) => Ok(http::Response::builder()
            .status(200)
            .body(Some("Message published".into()))?),
        Err(e) => Ok(http::Response::builder()
            .status(500)
            .body(Some(format!("Error publishing message: {:?}", e).into()))?)
    };
    
}
