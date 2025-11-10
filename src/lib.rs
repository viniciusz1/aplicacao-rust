use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

/// Handler que retorna a mensagem "Hello, world!"
pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

/// Estrutura para receber os parâmetros de operações matemáticas
#[derive(Debug, Deserialize)]
pub struct MathParams {
    a: f64,
    b: f64,
}

/// Estrutura para a resposta das operações matemáticas
#[derive(Debug, Serialize)]
pub struct MathResult {
    operation: String,
    a: f64,
    b: f64,
    result: f64,
}

/// Enum personalizado para erros da aplicação
#[derive(Debug)]
pub enum AppError {
    MissingParameters,
    InvalidNumber,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::MissingParameters => {
                (StatusCode::BAD_REQUEST, "Parâmetros 'a' e 'b' são obrigatórios")
            }
            AppError::InvalidNumber => {
                (StatusCode::BAD_REQUEST, "Parâmetros devem ser números válidos")
            }
        };
        (status, message).into_response()
    }
}

/// Handler para adição
/// Recebe query params: ?a=5&b=3
/// Retorna: {"operation": "addition", "a": 5, "b": 3, "result": 8}
pub async fn add(Query(params): Query<MathParams>) -> Json<MathResult> {
    let result = params.a + params.b;
    Json(MathResult {
        operation: "addition".to_string(),
        a: params.a,
        b: params.b,
        result,
    })
}

/// Handler para subtração
/// Recebe query params: ?a=10&b=3
/// Retorna: {"operation": "subtraction", "a": 10, "b": 3, "result": 7}
pub async fn subtract(Query(params): Query<MathParams>) -> Json<MathResult> {
    let result = params.a - params.b;
    Json(MathResult {
        operation: "subtraction".to_string(),
        a: params.a,
        b: params.b,
        result,
    })
}

/// Cria e configura o router da aplicação
/// 
/// Esta função é pública para permitir testes de integração
/// sem necessidade de iniciar o servidor completo
pub fn create_app() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/add", get(add))
        .route("/subtract", get(subtract))
}
