<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

class ClientManager {
    private $cntx;
    private $clients = [];
    private $seq = 1;

    public function __construct($cntx) {
        $this->cntx = $cntx;
    }

    public function nextSeq() {
        if ($this->seq === PHP_INT_MAX) $this->seq = 1;
        return $this->seq++;
    }

    public function onOpen($serv, $fd) {
        if (!isset($this->clients[$fd]))
            $this->clients[$fd] = new Client($this->cntx, $fd);
    }

    public function onMessage($serv, $frame) {
        $this->clients[$frame->fd]
             ->onMessage($frame->data);
    }

    public function onClose($serv, $fd, $reactorId) {
        unset($this->clients[$fd]);
    }

    public function onPacket($packet) {
        foreach ($this->clients as $client) {
            if ($client->onPacket($packet))
                break;
        }
    }
}
