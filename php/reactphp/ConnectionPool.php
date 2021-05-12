<?php
use React\Socket\ConnectionInterface;

class ConnectionPool {
    private $connections;

    public function __construct() {
        $this->connections = new SplObjectStorage();
    }

    public function add(ConnectionInterface $connection) {
        $connection->write('Enter you name: ');
        $this->initEvents($connection);
        $this->setConnectionData($connection, []);
    }

    private function initEvents(ConnectionInterface $connection) {
        $connection->on('data', function($data) use ($connection) {
            $connectionData = $this->getConnectionData($connection);
            if (!$connectionData) {
                $this->addNewMember($data, $connection);
                return;
            }
            $name = $connectionData['name'];
            $this->sendall("$name: $data", $connection);
        });

        $connection->on('close', function() use ($connection) {
            $connectionData = $this->getConnectionData($connection);
            $name = $connectionData['name'] ?? 'Unknown';
            $this->connections->offsetUnset($connection);
            $this->sendAll("User $name leaves the chat\n", $connection);
        });
    }

    private function getConnectionData(ConnectionInterface $connection) {
        return $this->connections->offsetGet($connection);
    }

    private function setConnectionData(ConnectionInterface $connection, $connectionData) {
        $this->connections->offsetSet($connection, $connectionData);
        return $this;
    }

    private function addNewMember($name, ConnectionInterface $connection) {
        $name = str_replace(["\n", "\r"], "", $name);
        $this->setConnectionData($connection, ['name' => $name]);
        $this->sendAll("User $name joins the chat\n", $connection);
    }

    private function sendAll($data, $except) {
        foreach ($this->connections as $connection) {
            if ($connection !== $except) $connection->write($data);
        }
    }
}