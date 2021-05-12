openssl genrsa --help
openssl genrsa -des3 -out www.hzw.org.key 2048
cat www.hzw.org.key
openssl rsa -check -in www.hzw.org.key

openssl genrsa -out www.zhiwenhe.org.key
cat www.zhiwenhe.org.key
openssl rsa -check -in www.zhiwenhe.org.key

openssl req -key www.zhiwenhe.org.key -new -out www.zhiwenhe.org.csr
cat www.zhiwenhe.org.csr

openssl req -key www.hzw.org.key -new -out www.hzw.org.csr
cat www.hzw.org.csr

openssl x509 -signkey www.hzw.org.key -in www.hzw.org.csr -req -out www.hzw.org.crt
cat www.hzw.org.crt

openssl x509 -signkey www.zhiwenhe.org.key -in www.zhiwenhe.org.csr -req -days 365 -out www.zhiwenhe.org.crt
cat www.zhiwenhe.org.crt

openssl x509 -in www.zhiwenhe.org.crt -text
openssl x509 -in www.zhiwenhe.org.crt -text -noout

openssl req -in www.zhiwenhe.org.csr -text -noout
