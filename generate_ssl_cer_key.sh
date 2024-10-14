rm key.pem
rm cert.pem

#mkcert 127.0.0.1
mkcert 192.168.1.X

mv 127.0.0.1-key.pem key.pem
mv 127.0.0.1.pem cert.pem
