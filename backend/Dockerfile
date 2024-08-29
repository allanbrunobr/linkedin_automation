# Dockerfile para o backend

# Etapa de compilação
FROM rust:bullseye as builder

# Instalar pacotes necessários, incluindo certificados CA
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    ca-certificates

# Cria um diretório de trabalho
WORKDIR /usr/src/backend

# Copia o código fonte para dentro do container
COPY . .

# Compila a aplicação em modo release
RUN cargo build --release

# Etapa de produção
FROM ubuntu:20.04

# Instalar pacotes necessários, incluindo certificados CA
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Cria um diretório de trabalho para o binário
WORKDIR /usr/src/backend

# Copia os binários compilados da fase anterior para a nova imagem
COPY --from=builder /usr/src/backend/target/release/web_server /usr/local/bin/web_server
COPY --from=builder /usr/src/backend/target/release/scheduler /usr/local/bin/scheduler

# Copia os arquivos de configuração
COPY config.toml /usr/src/backend/

# Expõe a porta onde o backend vai rodar
EXPOSE 8080

# Comando para rodar o servidor e o scheduler em paralelo
CMD ["sh", "-c", "web_server & scheduler"]