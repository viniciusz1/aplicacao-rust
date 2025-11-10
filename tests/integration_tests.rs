use aplicacao_rust::create_app;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
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
