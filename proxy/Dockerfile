FROM nginx:alpine

RUN apk add --no-cache openssl

# allow overriding CN for real deployments; default = 'localhost'
ARG CN=localhost
RUN mkdir -p /etc/nginx/certs \
 && openssl req -x509 -newkey rsa:4096 -nodes \
      -keyout /etc/nginx/certs/key.pem \
      -out /etc/nginx/certs/cert.pem \
      -days 365 \
      -subj "/C=US/ST=Local/L=Local/O=Local/CN=${CN}"

COPY conf.d/default.conf /etc/nginx/conf.d/default.conf

EXPOSE 80 443 8000 3000