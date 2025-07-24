FROM nginx:alpine

# install openssl for cert generation
RUN apk add --no-cache openssl

# generate a self-signed cert for CN=localhost
RUN mkdir -p /etc/nginx/certs \
 && openssl req -x509 -newkey rsa:4096 -nodes \
      -keyout /etc/nginx/certs/key.pem \
      -out /etc/nginx/certs/cert.pem \
      -days 365 \
      -subj "/C=US/ST=Local/L=Local/O=Local/CN=localhost"

# copy our custom nginx config
COPY conf.d/default.conf /etc/nginx/conf.d/default.conf

# expose the custom ports inside the container
EXPOSE 8081 8444