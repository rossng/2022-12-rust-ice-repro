use server::web_server;

#[tokio::main]
pub async fn main() -> () {
    web_server(8000).await;
}
