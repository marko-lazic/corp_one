# Generate a private key

openssl genrsa -out server.key 2048

# Generate a self-signed certificate in PEM format

openssl req -new -x509 -key server.key -out server.pem -days 365
