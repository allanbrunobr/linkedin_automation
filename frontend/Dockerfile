# Dockerfile para o frontend

# Etapa de construção
FROM node:18.20.3 as builder

# Cria um diretório de trabalho
WORKDIR /usr/src/frontend

# Copia o package.json e o package-lock.json
COPY package*.json ./

# Instala as dependências
RUN npm install

# Copia o código fonte para dentro do container
COPY . .

# Gera uma versão otimizada da aplicação React para produção
RUN npm run build

# Etapa de produção
FROM nginx:alpine

# Copia os arquivos de build para o diretório padrão do nginx
COPY --from=builder /usr/src/frontend/build /usr/share/nginx/html

# Expõe a porta onde o frontend vai rodar
EXPOSE 80

# Comando padrão para rodar o nginx
CMD ["nginx", "-g", "daemon off;"]
