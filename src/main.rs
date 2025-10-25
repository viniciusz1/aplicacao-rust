use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    // cria a aplicação com uma rota GET /
    let app = Router::new().route("/", get(hello_world));

    // endereço onde o servidor vai escutar
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Servidor rodando em http://{}", addr);

    // serve a aplicação
    // axum 0.7 usa axum::serve + TcpListener
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("não consegui bindar na porta 8080");

    axum::serve(listener, app)
        .await
        .expect("erro ao rodar o servidor");
}
