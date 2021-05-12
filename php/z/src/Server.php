<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

use Monolog\Logger;
use Monolog\Handler\StreamHandler;
use Hzw\Task\DbConnect;
use Hzw\Task\Tasks;
use Hzw\Task\OfflineDevices;
use Hzw\Task\LoadDevices;

class Server {
    private $serv;
    private $logger;
    private $databaseMgr;

    private $deviceMgr;
    private $clientMgr;

    private $enabled = false;

    public function __construct() {
        foreach ([Config::LOG_DIR, Config::DATA_DIR] as $dir) {
            if (!is_dir($dir)) {
                mkdir($dir, 0777, true);
                chown($dir, Config::USER);
                chgrp($dir, Config::GROUP);
            }
        }

        $serv = new \swoole_websocket_server('0.0.0.0', Config::WS_PORT);
        $serv->addListener('0.0.0.0', Config::UDP_PORT, SWOOLE_SOCK_UDP);

        $serv->set([
            'daemonize' => true,
            'pid_file' => Config::DATA_DIR . '/iot.pid',
            'log_file' => Config::LOG_DIR . '/log_file',
            'worker_num' => 1,
            'task_worker_num' => 2,
            'user' => Config::USER,
            'group' => Config::GROUP
        ]);

        $serv->on('Start', [$this, 'onStart']);
        $serv->on('Shutdown', [$this, 'onShutdown']);

        $serv->on('ManagerStart', [$this, 'onManagerStart']);
        $serv->on('WorkerError', [$this, 'onWorkerError']);
        $serv->on('ManagerStop', [$this, 'onManagerStop']);

        $serv->on('WorkerStart', [$this, 'onWorkerStart']);
        $serv->on('Open', [$this, 'onOpen']);
        $serv->on('Message', [$this, 'onMessage']);
        $serv->on('Close', [$this, 'onClose']);
        $serv->on('Packet', [$this, 'onPacket']);
        $serv->on('Finish', [$this, 'onFinish']);
        $serv->on('WorkerStop', [$this, 'onWorkerStop']);

        $serv->on('Task', [$this, 'onTask']);

        $this->serv = $serv;
    }

    function getServ() {
        return $this->serv;
    }

    function getLogger() {
        return $this->logger;
    }

    function getDatabaseMgr() {
        return $this->databaseMgr;
    }

    function getDeviceMgr() {
        return $this->deviceMgr;
    }

    function getClientMgr() {
        return $this->clientMgr;
    }

    function enable() {
        $this->enabled = true;
    }

    function disable() {
        $this->enabled = false;
    }

    function enabled() {
        return $this->enabled;
    }

    function createLogger($filename) {
        $stream = new StreamHandler(Config::LOG_DIR . "/$filename", Config::LOGGER_LEVEL);
        $formatter = new LineFormatter();
        $stream->setFormatter($formatter);

        $this->logger = new Logger('main');
        $this->logger->pushHandler($stream);
    }

    function start() {
        $this->serv->start();
    }

    public function sendto($address, $data) {
        $this->logger->debug('sendto', ['address' => $address, 'data' => $data]);
        if (isset($address['devaddr'])) {
            $this->loraClient->sendto($address, $data);
        } else {
            $this->serv->sendto($address['ip'], $address['port'], $data);
        }
    }

    public function multicast($addresses, $data) {
        $this->logger->debug('multicast', ['addresses' => $addresses, 'data' => $data]);
        if (isset($addresses[0]['devaddr'])) {
            $this->loraClient->multicast($addresses, $data);
        }
    }

    function push($fd, $data) {
        $this->logger->debug('push', ['fd' => $fd, 'data' => $data]);
        $this->serv->push($fd, $data);
    }

    function onStart($serv) {
        swoole_set_process_name(Config::NAME . ' master');
        $this->createLogger('master');
        $this->logger->debug('onStart', ['master_pid' => $serv->master_pid]);
    }

    function onShutdown($serv) {
        $this->logger->debug('onShutdown', ['master_pid' => $serv->master_pid]);
    }

    function onManagerStart($serv) {
        swoole_set_process_name(Config::NAME . ' manager');
        $this->createLogger('manager');
        $this->logger->debug('onManagerStart', ['manager_pid' => $serv->manager_pid]);
    }

    public function onWorkerError($serv, $worker_id, $worker_pid, $exit_code, $signal) {
        $this->logger->critical('onWorkerError', [
            'manager_pid' => $serv->manager_pid,
            'worker_pid' => $worker_pid,
            'worker_id' => $worker_id,
            'exit_code' => $exit_code,
            'signal' => $signal
        ]);
    }

    public function onManagerStop($serv) {
        $this->logger->debug('onManagerStop', ['manager_pid' => $serv->manager_pid]);
    }

    public function onWorkerStart($serv, $worker_id) {
        $name = "worker${worker_id}";
        if ($serv->taskworker) {
            $name = "taskworker${worker_id}";
        }

        swoole_set_process_name(Config::NAME . " $name");

        $this->createLogger($name);

        $this->logger->debug('onWorkerStart', [
            'worker_pid' => $serv->worker_pid,
            'worker_id' => $worker_id,
            'taskworker' => $serv->taskworker
        ]);

        Debug::init($this);
        TaskManager::init($this);

        $this->databaseMgr = $databaseMgr = new DatabaseManager(
            Config::DSN, Config::DB_USER, Config::DB_PW, $this->logger
        );
        $databaseMgr->addListener(DatabaseManager::CONNECTED, [TaskManager::class, 'onDbConnected']);
        $databaseMgr->addListener(DatabaseManager::DISCONNECTED, [TaskManager::class, 'onDbDisConnected']);

        if (!$serv->taskworker) {
            Packet::init();

            $this->deviceMgr = new DeviceManager($this);
            $this->clientMgr = new ClientManager($this);

            $this->loraClient = new LoraClient(
                $this, Config::LORA_SERVER, Config::LORA_APPEUI
            );
            $this->loraClient->connect();

            TaskManager::task(new DbConnect(), TaskManager::ALL_TASK_WORKERS);
            TaskManager::workerTask(new Tasks(
                [new OfflineDevices(), new LoadDevices()]
            ));
        }
    }

    public function onPacket($serv, $data, $clientInfo, $extraInfo = null) {
        $logger = $this->logger;
        if (!$this->enabled()) {
            $logger->critical('onPacket server is disabled.');
            return;
        }

        $source = $clientInfo;
        if (!isset($source['devaddr'])) {
            $source = ['ip' => $source['address'], 'port' => $source['port']];
        }
        $pcdt = time();
        $packet = Packet::unpack($data);
        $packet->setSource($source)
               ->setPcdt($pcdt)
               ->setExtraInfo($extraInfo);

        $logger->debug('onPacket', ['packet' => $packet, 'data' => $data]);

        if (!$packet->isValid('T')) return;

        if ($packet->needAck()) {
            $this->sendto($source, $packet->packAck());
        }

        $header = $packet->getHeader();
        if ($header->getSns()[0] === '00000000000000') return;
        if ($header->getDt() > $pcdt) {
            $logger->warning('device future time');
            $header->setDt($pcdt);
        }

        $this->deviceMgr->onPacket($packet);
        $this->clientMgr->onPacket($packet);
        TaskManager::onPacket($packet);
    }

    public function onOpen($serv, $req) {
        $this->logger->debug('onOpen', ['fd' => $req->fd]);
        $this->clientMgr->onOpen($serv, $req->fd);
    }

    public function onMessage($serv, $frame) {
        $fd = $frame->fd;
        $data = $frame->data;
        $this->logger->debug('onMessage', ['fd' => $fd, 'data' => $data]);
        $this->clientMgr->onMessage($serv, $frame);
    }

    public function onClose($serv, $fd, $reactorId) {
        $this->logger->debug('onClose', ['fd' => $fd, 'reactorId' => $reactorId]);
        $this->clientMgr->onClose($serv, $fd, $reactorId);
    }

    public function onFinish($serv, $taskId, $data) {
        $this->logger->debug('onFinish', [
            'workerPid' => $serv->worker_pid,
            'workerId' => $serv->worker_id,
            'taskId' => $taskId,
            'data' => $data
        ]);

        TaskManager::onFinish($serv, $taskId, $data);
    }

    public function onWorkerStop($serv, $worker_id) {
        $this->logger->debug('onWorkerStop', [
            'worker_pid' => $serv->worker_pid,
            'worker_id' => $worker_id,
            'taskworker' => $serv->taskworker
        ]);
    }

    public function onTask($serv, $taskId, $srcWorkerId, $task) {
        $this->logger->debug('onTask', [
            'workerPid' => $serv->worker_pid,
            'workerId' => $serv->worker_id,
            'taskId' => $taskId,
            'srcWorkerId' => $srcWorkerId,
            'task' => $task
        ]);

        return TaskManager::onTask($serv, $taskId, $srcWorkerId, $task);
    }
}
