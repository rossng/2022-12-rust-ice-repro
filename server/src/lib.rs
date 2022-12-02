use core::convert::Infallible;
use warp::Filter;

pub async fn web_server(port: u16) {
    let cors = warp::cors();

    let routes = warp::fs::dir("www")
        .or(warp::get().and_then(serve_foo))
        .with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

async fn serve_foo() -> Result<impl warp::Reply, Infallible> {
    let contents = String::new();
    Ok(contents)
}
