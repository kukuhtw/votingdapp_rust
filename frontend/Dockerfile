# ---- Build
# voting-dapp/frontend/Dockerfile

# ---- Build
FROM node:20-alpine AS build
WORKDIR /app

# copy manifest & lock
COPY package*.json ./

# install SEMUA deps (termasuk dev) karena butuh vite untuk build
RUN npm ci

# copy source
COPY . .

# build
RUN npm run build

# ---- Runtime (Nginx + SPA fallback)
FROM nginx:alpine
WORKDIR /usr/share/nginx/html
COPY --from=build /app/dist ./

# SPA fallback untuk vue-router history mode
RUN printf 'server {\n\
  listen 80;\n\
  root /usr/share/nginx/html;\n\
  location / {\n\
    try_files $uri $uri/ /index.html;\n\
  }\n\
}\n' > /etc/nginx/conf.d/default.conf

EXPOSE 80
