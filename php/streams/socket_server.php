<?php
echo "before stream_socket_server\n";
$server = stream_socket_server('tcp://0.0.0.0:8000', $errno, $errstr);
echo "after stream_socket_server\n";
//stream_set_blocking($server, 0);
echo "before stream_socket_accept\n";
$client = stream_socket_accept($server);
echo "after stream_socket_accept\n";
