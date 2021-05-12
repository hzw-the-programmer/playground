<?php
register_shutdown_function(function () {
    echo getmypid() . " shutdown 1\n";
    //exit();
});

swoole_process::signal(SIGCHLD, function ($sig) {
    while ($ret = swoole_process::wait(false)) {
        var_dump($ret);
    }
    //swoole_event_exit();
});

register_shutdown_function(function () {
    echo getmypid() . " shutdown 2\n";
    //exit();
});

$process = new swoole_process(function ($process) {
    for ($i = 0; $i < 10; $i++) {
        //sleep(1);
        $process->write($i);
    }
}); //, false, 1);

register_shutdown_function(function () {
    echo getmypid() . " shutdown 3\n";
    //exit();
});

swoole_event_add($process->pipe, function ($pipe) use ($process) {
    echo "before sleep\n";
    sleep(1);
    echo $process->read() . "\n";
});

register_shutdown_function(function () {
    echo getmypid() . " shutdown 4\n";
    //exit();
});

$process->start();

register_shutdown_function(function () {
    echo getmypid() . " shutdown 5\n";
    //exit();
});
