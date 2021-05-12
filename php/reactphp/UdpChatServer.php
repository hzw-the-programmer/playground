<?php
require __DIR__ . '/vendor/autoload.php';

use React\Datagram\Socket;

class UdpChatServer {
    public function __construct($address, $loop) {
        $this->address = $address;
        $this->loop = $loop;
    }

    public function init() {
        $factory = new React\Datagram\Factory($this->loop);

        $factory->createServer($this->address)
            ->then(
                function(Socket $server) {
                    echo "Listening on {$this->address}\n";
                    $this->server = $server;
                    $server->on('message', [$this, 'process']);
                },
                function(Exception $e) {
                    echo "Error: {$e->getMessage()}\n";
                }
            );
    }

    function process($message, $address, $server) {
        $message = json_decode($message, true);

        switch ($message['type']) {
            case 'enter':
                $this->addClient($address, $message['name']);
                break;
            case 'leave':
                $this->removeClient($address);
                break;
            default:
                $this->sendMessage($message['message'], $address);
                break;
        }
    }

    private function addClient($address, $name) {
        if (isset($this->clients[$address])) return;

        $this->clients[$address] = $name;
        $this->broadcast("$name enters chat", $address);
    }

    private function removeClient($address) {
        $name = $this->clients[$address] ?? 'Unknown';

        unset($this->clients[$address]);
        $this->broadcast("$name leaves chat");
    }

    private function sendMessage($message, $address) {
        $name = $this->clients[$address] ?? 'Unknown';
        $this->broadcast("$name: $message", $address);
    }

    private function broadcast($message, $except = null) {
        foreach ($this->clients as $address => $name) {
            if ($address !== $except) $this->server->send($message, $address);
        }
    }
}

$loop = React\EventLoop\Factory::create();

$udpChatServer = new UdpChatServer('0.0.0.0:8000', $loop);
$udpChatServer->init();

$loop->run();
