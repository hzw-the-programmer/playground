<?php
use Swoole\Coroutine as co;

co::create(function() {
    echo "1\n";
    $mysql = new co\MySQL();
    echo "2\n";
    $mysql->connect([
        'host' => '127.0.0.1',
        'port' => 3306,
        'user' => 'root',
        'password' => 'asdF@123',
        'database' => 'menagerie'
    ]);
    echo "4\n";
    co::create(function() use ($mysql) {
        echo "5\n";
        $res = $mysql->query('SELECT * FROM pet');
        echo "7\n";
        var_dump($res);
    });
    echo "6\n";
});

echo "3\n";
