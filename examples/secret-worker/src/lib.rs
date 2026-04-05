use wstd::http::{Body, Request, Response, Result, StatusCode};

#[wstd::http_server]
async fn main(req: Request<Body>) -> Result<Response<Body>> {
    match req.uri().path() {
        "/" => {
            let greeting = std::env::var("GREETING").unwrap_or_else(|_| "missing".to_owned());
            Ok(Response::new(format!("greeting={greeting}\n").into()))
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("not found\n".into())
            .expect("response build")),
    }
}
