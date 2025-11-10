use aplicacao_rust::create_app;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Cria a aplicação usando a função do módulo lib
    let app = create_app();

    // Endereço onde o servidor vai escutar
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Servidor rodando em http://{}", addr);

    // Serve a aplicação
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("não consegui bindar na porta 8080");

    axum::serve(listener, app)
        .await
        .expect("erro ao rodar o servidor");
}
