<?php
class EchoServer
{
    const WEBSOCKET_PORT0 = 9501;
    const WEBSOCKET_PORT1 = 9502;
    const HTTP_PORT = 9503;
    const TCP_PORT = 9504;
    const EOF_CHECK_PORT = 9505;
    const EOF_SPLIT_PORT = 9506;
    const LENGTH_CHECK_PORT = 9507;

    function __construct()
    {
        $serv = new swoole_websocket_server('0.0.0.0', EchoServer::WEBSOCKET_PORT0);

        $serv->set([
            'daemonize' => 1,
            'log_file' => '/tmp/echoserver',
            'pid_file' => '/tmp/echoserver.pid',
            'task_worker_num' => 1,
        ]);

        $serv->on('Start', [$this, 'onStart']);
        $serv->on('Shutdown', [$this, 'onShutdown']);

        $serv->on('ManagerStart', [$this, 'onManagerStart']);
        $serv->on('ManagerStop', [$this, 'onManagerStop']);
        $serv->on('WorkerError', [$this, 'onWorkerError']);

        $serv->on('WorkerStart', [$this, 'onWorkerStart']);
        $serv->on('WorkerStop', [$this, 'onWorkerStop']);

        $serv->on('Connect', [$this, 'onConnect']);

        $serv->on('Open', [$this, 'onOpen']);
        $serv->on('Message', [$this, 'onMessage']);

        $serv->on('Request', [$this, 'onRequest']);

        $serv->on('Receive', [$this, 'onReceive']);
        
        $serv->on('Close', [$this, 'onClose']);

        $serv->on('Task', [$this, 'onTask']);
        $serv->on('Finish', [$this, 'onFinish']);

        $this->serv = $serv;

        $websocket = $serv->listen('0.0.0.0', EchoServer::WEBSOCKET_PORT1, SWOOLE_SOCK_TCP);

        $http = $serv->listen('0.0.0.0', EchoServer::HTTP_PORT, SWOOLE_SOCK_TCP);
        $http->set([
            'open_http_protocol' => true
        ]);

        $tcp = $serv->listen('0.0.0.0', EchoServer::TCP_PORT, SWOOLE_SOCK_TCP);
        $tcp->set([
            'open_websocket_protocol' => false
        ]);

        // echo -n 'ahzwbhzwchzw' | netcat 127.0.0.1 9505
        // echo 'ahzwbhzwchzw' | netcat 127.0.0.1 9505
        $eof_check = $serv->listen('0.0.0.0', EchoServer::EOF_CHECK_PORT, SWOOLE_SOCK_TCP);
        $eof_check->set([
            'open_eof_check' => true,
            'package_eof' => 'hzw'
        ]);

        // echo 'ahzwbhzwchzw' | netcat 127.0.0.1 9506
        $eof_split = $serv->listen('0.0.0.0', EchoServer::EOF_SPLIT_PORT, SWOOLE_SOCK_TCP);
        $eof_split->set([
            'open_eof_split' => true,
            'package_eof' => 'hzw'
        ]);

        // php -r "echo hex2bin('4455000344554455334400034433443344');" | nc localhost 9507
        // echo -e '\x44\x55\x00\x03\x44\x55\x44\x55\x33\x44\x00\x03\x44\x33\x44\x33\x44' | nc localhost 9507
        $length_check = $serv->listen('0.0.0.0', EchoServer::LENGTH_CHECK_PORT, SWOOLE_SOCK_TCP);
        $length_check->set([
            'open_length_check' => true,
            'package_length_offset' => 2,
            'package_length_type' => 'n',
            'package_body_offset' => 5
        ]);
    }

    function start()
    {
        $this->serv->start();
    }

    function onStart($serv)
    {
        echo "onStart: pid = {$serv->master_pid}\n";
    }

    function onShutdown($serv)
    {
        echo "onShutdown: pid = {$serv->master_pid}\n";
    }

    function onManagerStart($serv)
    {
        echo "onManagerStart: pid = {$serv->manager_pid}\n";
    }

    function onManagerStop($serv)
    {
        echo "onManagerStop: pid = {$serv->manager_pid}\n";
    }

    function onWorkerError($serv, $workerId, $workerPid, $exitCode, $signal)
    {
        echo "onWorkerError: id = $workerId, pid = $workerPid, exitCode = $exitCode, signal = $signal\n";
    }

    function onWorkerStart($serv, $workerId)
    {
        echo "onWorkerStart: id = $workerId, pid = {$serv->worker_pid}\n";
    }

    function onWorkerStop($serv, $workerId)
    {
        echo "onWorkerStop: id = $workerId, pid = {$serv->worker_pid}\n";
    }

    function onConnect($serv, $fd, $reactorId)
    {
        echo "onConnect: fd $fd from reactor $reactorId connected\n";
    }

    function onOpen($serv, $request)
    {
        echo "onOpen: fd {$request->fd} handshake success\n";
    }

    function onMessage($serv, $frame)
    {
        echo "onMessage: fd {$frame->fd}: {$frame->data}\n";
        $serv->push($frame->fd, $frame->data);
    }

    function onRequest($req, $res) {
        echo "onRequest:\n";
        $res->end('<h1>Hello HZW</h1>');
    }

    // websocket does not call onReceive, it calls onMessage.
    // http does not call onReceive, it calls onRequest.
    function onReceive($serv, $fd, $reactorId, $data)
    {
        $conn = $serv->connection_info($fd);
        if ($conn['server_port'] === EchoServer::LENGTH_CHECK_PORT) {
            $data = bin2hex($data);
        }
        echo "onReceive: fd $fd from reactor $reactorId received data: $data\n";
        $serv->send($fd, $data);
    }

    function onClose($serv, $fd, $reactorId)
    {
        echo "onClose: fd $fd from reactor $reactorId closed\n";
    }

    function onTask($serv, $taskId, $srcWorkerId, $data)
    {
        echo "onTask: task $taskId is delivered to " .
             "task worker {$serv->worker_id}(pid={$serv->worker_pid}) " .
             "from worker {$srcWorkerId}\n";
    }

    function onFinish($serv, $taskId, $data)
    {
        echo "onFinish: task $taskId finished\n";
    }
}

$serv = new EchoServer();
$serv->start();
