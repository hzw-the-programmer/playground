<?php
// openssl genrsa -des3 -out localhost.key 1024
// openssl req -new -key localhost.key -out localhost.csr
// openssl x509 -req -days 365 -in localhost.csr -signkey localhost.key -out localhost.crt

// ./configure --enable-openssl
// make clean
// make

$serv = new Swoole\Http\Server('0.0.0.0', 9501, SWOOLE_PROCESS, SWOOLE_SOCK_TCP | SWOOLE_SSL);
$serv->set([
    'ssl_cert_file' => './localhost.crt',
    'ssl_key_file' => './localhost.key'
]);
$serv->on('Request', function($req, $res) {
    $res->end('<h1>Hello Swoole. #' . rand(1000, 9999) . '</h1>');
});

$serv->start();
