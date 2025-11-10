use axum::{routing::get, Router};

/// Handler que retorna a mensagem "Hello, world!"
pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

/// Cria e configura o router da aplicação
/// 
/// Esta função é pública para permitir testes de integração
/// sem necessidade de iniciar o servidor completo
pub fn create_app() -> Router {
    Router::new().route("/", get(hello_world))
}
