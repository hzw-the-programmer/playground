<?php
$client = new swoole_client(SWOOLE_SOCK_UDP);
$client->sendto("127.0.0.1", 9501, "zhiwenhe");
