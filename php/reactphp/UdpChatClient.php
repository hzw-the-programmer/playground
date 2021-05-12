<?php
require __DIR__ . '/vendor/autoload.php';

class UdpChatClient {
    private $address;
    private $loop;
    private $client;
    private $name;

    public function __construct($address, $loop) {
        $this->address = $address;
        $this->loop = $loop;
    }

    public function init() {
        $stdin = new React\Stream\ReadableResourceStream(STDIN, $this->loop);
        $stdin->on('data', [$this, 'processInput']);

        $factory = new React\Datagram\Factory($this->loop);
        $factory->createClient($this->address)
            ->then(
                [$this, 'initClient'],
                function(Exception $e) {
                    echo "Error: {$e->getMessage()}\n";
                }
            );
    }

    function processInput($data) {
        $data = trim($data);

        if (!$this->name) {
            $this->name = $data;
            $this->sendData('', 'enter');
            return;
        }

        if ($data === ':exit') {
            $this->sendData('', 'leave');
            $this->client->end();
            return;
        }

        $this->sendData($data);
    }

    function initClient($client) {
        $this->client = $client;

        $client->on('message', function($message, $address, $client) {
            echo "$message\n";
        });

        $client->on('close', function() {
            $this->loop->stop();
        });

        echo "Enter your name: ";
    }

    private function sendData($message, $type = 'message') {
        $data = [
            'type' => $type,
            'name' => $this->name,
            'message' => $message,
        ];

        $this->client->send(json_encode($data));
    }
}

$loop = React\EventLoop\Factory::create();

$udpChatClient = new UdpChatClient('127.0.0.1:8000', $loop);
$udpChatClient->init();

$loop->run();
