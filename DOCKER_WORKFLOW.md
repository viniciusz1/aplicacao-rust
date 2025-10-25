# Docker Workflow - Instruções

## Configuração do Forgejo Actions

Este workflow automaticamente constrói e publica a imagem Docker da aplicação Rust no registry de contêineres do Forgejo.

### O que o workflow faz:

1. **Triggers**: Executa quando há push para `main`/`master`, quando tags são criadas (v\*), ou em pull requests
2. **Build**: Constrói a imagem Docker usando o Dockerfile
3. **Publish**: Publica a imagem no registry do Forgejo

### Tags geradas automaticamente:

- `latest` - para commits na branch principal
- `v1.0.0` - para tags semver
- `main-sha123456` - para commits específicos
- `pr-123` - para pull requests

### Como usar a imagem publicada:

```bash
# Baixar e rodar a imagem
docker pull <seu-forgejo-url>/<seu-usuario>/aplicacao-rust:latest
docker run -p 8080:8080 <seu-forgejo-url>/<seu-usuario>/aplicacao-rust:latest
```

### Configuração necessária:

O workflow usa automaticamente o token `GITHUB_TOKEN` (que no Forgejo é equivalente ao token da instância). Não é necessário configurar secrets adicionais para publicar no registry do próprio Forgejo.

### Para usar Docker Hub ou outro registry:

Se preferir publicar no Docker Hub ou outro registry, modifique o workflow:

```yaml
- name: Log in to Docker Hub
  uses: docker/login-action@v3
  with:
    username: ${{ secrets.DOCKERHUB_USERNAME }}
    password: ${{ secrets.DOCKERHUB_TOKEN }}
```

E adicione os secrets `DOCKERHUB_USERNAME` e `DOCKERHUB_TOKEN` nas configurações do repositório no Forgejo.

### Testando localmente:

```bash
# Build da imagem
docker build -t aplicacao-rust:test .

# Rodar
docker run -p 8080:8080 aplicacao-rust:test

# Testar
curl http://localhost:8080
```
