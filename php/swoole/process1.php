<?php
$chan = new Swoole\Channel(256 * 1024);
$n = 100000;
$bytes = 0;

$child = new Swoole\Process(function($process) use ($chan, $n, $bytes) {
    for ($i = 0; $i < $n; $i++) {
        $data = str_repeat("A", rand(100, 200));
        if ($chan->push($data) == false) {
            echo "channel full\n";
            usleep(1000);
            $i--;
            continue;
        }
        $bytes += strlen($data);
    }
    echo "total push bytes: $bytes\n";
    var_dump($chan->stats());
});
$child->start();

for ($i = 0; $i < $n; $i++) {
    if (($data = $chan->pop()) == false) {
        echo "channel empty\n";
        usleep(1000);
        $i--;
        continue;
    }
    $bytes += strlen($data);
}
echo "total pop bytes: $bytes\n";
var_dump($chan->stats());
