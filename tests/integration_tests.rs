use aplicacao_rust::create_app;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::Value;
use tower::ServiceExt; // para oneshot()

#[tokio::test]
async fn test_hello_world_route() {
    // Cria a aplicação
    let app = create_app();

    // Cria uma requisição GET para /
    let request = Request::builder()
        .uri("/")
        .body(Body::empty())
        .unwrap();

    // Envia a requisição e obtém a resposta
    let response = app.oneshot(request).await.unwrap();

    // Verifica se o status é 200 OK
    assert_eq!(response.status(), StatusCode::OK);

    // Lê o corpo da resposta
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    // Verifica se o corpo contém a mensagem esperada
    assert_eq!(body_str, "Hello, world!");
}

#[tokio::test]
async fn test_not_found_route() {
    // Cria a aplicação
    let app = create_app();

    // Cria uma requisição GET para uma rota que não existe
    let request = Request::builder()
        .uri("/nao-existe")
        .body(Body::empty())
        .unwrap();

    // Envia a requisição e obtém a resposta
    let response = app.oneshot(request).await.unwrap();

    // Verifica se o status é 404 NOT FOUND
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_method_not_allowed() {
    // Cria a aplicação
    let app = create_app();

    // Cria uma requisição POST para / (só aceita GET)
    let request = Request::builder()
        .method("POST")
        .uri("/")
        .body(Body::empty())
        .unwrap();

    // Envia a requisição e obtém a resposta
    let response = app.oneshot(request).await.unwrap();

    // Verifica se o status é 405 METHOD NOT ALLOWED
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_multiple_requests() {
    // Testa múltiplas requisições para garantir que o app é stateless
    for _ in 0..5 {
        let app = create_app();
        
        let request = Request::builder()
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        
        assert_eq!(body_str, "Hello, world!");
    }
}

#[tokio::test]
async fn test_response_content_type() {
    // Testa se o content-type está correto
    let app = create_app();
    
    let request = Request::builder()
        .uri("/")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    // Axum retorna text/plain por padrão para &str
    let content_type = response.headers().get("content-type");
    assert!(content_type.is_some());
}

// ========== TESTES DO ENDPOINT /add ==========

#[tokio::test]
async fn test_add_positive_numbers() {
    let app = create_app();
    
    let request = Request::builder()
        .uri("/add?a=5&b=3")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(json["operation"], "addition");
    assert_eq!(json["a"], 5.0);
    assert_eq!(json["b"], 3.0);
    assert_eq!(json["result"], 8.0);
}

#[tokio::test]
async fn test_add_negative_numbers() {
    let app = create_app();
    
    let request = Request::builder()
        .uri("/add?a=-10&b=-5")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(json["operation"], "addition");
    assert_eq!(json["a"], -10.0);
    assert_eq!(json["b"], -5.0);
    assert_eq!(json["result"], -15.0);
}

#[tokio::test]
async fn test_add_decimal_numbers() {
    let app = create_app();
    
    let request = Request::builder()
        .uri("/add?a=2.5&b=3.7")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(json["operation"], "addition");
    assert_eq!(json["a"], 2.5);
    assert_eq!(json["b"], 3.7);
    assert_eq!(json["result"], 6.2);
}

#[tokio::test]
async fn test_add_zero() {
    let app = create_app();
    
    let request = Request::builder()
        .uri("/add?a=0&b=0")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(json["result"], 0.0);
}

#[tokio::test]
async fn test_add_missing_parameter_a() {
    let app = create_app();
    
    let request = Request::builder()
        .uri("/add?b=5")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    // Deve retornar erro 400 BAD REQUEST
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_add_missing_parameter_b() {
    let app = create_app();
    
    let request = Request::builder()
        .uri("/add?a=5")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    // Deve retornar erro 400 BAD REQUEST
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_add_invalid_parameter() {
    let app = create_app();
    
    let request = Request::builder()
        .uri("/add?a=abc&b=5")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    // Deve retornar erro 400 BAD REQUEST
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// ========== TESTES DO ENDPOINT /subtract ==========

#[tokio::test]
async fn test_subtract_positive_numbers() {
    let app = create_app();
    
    let request = Request::builder()
        .uri("/subtract?a=10&b=3")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(json["operation"], "subtraction");
    assert_eq!(json["a"], 10.0);
    assert_eq!(json["b"], 3.0);
    assert_eq!(json["result"], 7.0);
}

#[tokio::test]
async fn test_subtract_negative_result() {
    let app = create_app();
    
    let request = Request::builder()
        .uri("/subtract?a=5&b=10")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(json["operation"], "subtraction");
    assert_eq!(json["a"], 5.0);
    assert_eq!(json["b"], 10.0);
    assert_eq!(json["result"], -5.0);
}

#[tokio::test]
async fn test_subtract_decimal_numbers() {
    let app = create_app();
    
    let request = Request::builder()
        .uri("/subtract?a=7.5&b=2.3")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(json["operation"], "subtraction");
    assert_eq!(json["a"], 7.5);
    assert_eq!(json["b"], 2.3);
    // Usando aproximação devido a precisão de ponto flutuante
    assert!((json["result"].as_f64().unwrap() - 5.2).abs() < 0.001);
}

#[tokio::test]
async fn test_subtract_same_numbers() {
    let app = create_app();
    
    let request = Request::builder()
        .uri("/subtract?a=42&b=42")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(json["result"], 0.0);
}

#[tokio::test]
async fn test_subtract_with_zero() {
    let app = create_app();
    
    let request = Request::builder()
        .uri("/subtract?a=100&b=0")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(json["result"], 100.0);
}

#[tokio::test]
async fn test_subtract_missing_parameter_a() {
    let app = create_app();
    
    let request = Request::builder()
        .uri("/subtract?b=5")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    // Deve retornar erro 400 BAD REQUEST
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_subtract_missing_parameter_b() {
    let app = create_app();
    
    let request = Request::builder()
        .uri("/subtract?a=5")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    // Deve retornar erro 400 BAD REQUEST
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_subtract_invalid_parameter() {
    let app = create_app();
    
    let request = Request::builder()
        .uri("/subtract?a=10&b=xyz")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    // Deve retornar erro 400 BAD REQUEST
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_subtract_large_numbers() {
    let app = create_app();
    
    let request = Request::builder()
        .uri("/subtract?a=999999999&b=999999998")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(json["result"], 1.0);
}
