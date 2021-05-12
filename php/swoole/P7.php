<?php
class P7 {
    private $name;
    private $processes;

    function __construct($name, $count) {
        swoole_set_process_name("$name: master");
        $this->name = $name;
        for ($i = 0; $i < $count; $i++) {
            if ($process = static::createProcess($i, [$this, 'main'])) {
                $this->processes[$process->pid] = $process;
            }
        }
    }

    function start() {
        swoole_event_add(STDIN, [$this, 'task']);
        foreach ($this->processes as $process) {
            swoole_event_add($process->pipe, function($pipe) use ($process) {
                $this->onFinish($process);
            });
        }
    }

    function task($fd) {
        $code = trim(fgets($fd));
        $task = new Task($code);
        $process = $this->processes[array_rand($this->processes)];
        $process->write(serialize($task));
        echo sprintf("task(%d) is dispatched to process(%d)\n", $task->getId(), $process->pid);
    }

    function onFinish($process) {
        list($pid, $id, $ret) = unserialize($process->read());
        echo sprintf("process(%d)finish task(%d): %d\n", $pid, $id, $ret);
    }

    function main($process) {
        swoole_set_process_name("{$this->name}: child {$process->id}");
        swoole_event_add($process->pipe, function($pipe) use ($process) {
            $this->onTask($process);
        });
    }

    function onTask($process) {
        $task = unserialize($process->read());
        $process->write(serialize([getmypid(), $task->getId(), $task->process()]));
    }

    static function createProcess($id, $main) {
        $process = new swoole_process($main);
        $process->id = $id;
        if ($process->start()) {
            return $process;
        }
        return null;
    }
}

class Task implements Serializable {
    private $id;
    private $code;

    function __construct($code) {
        static $id = 1;
        $this->id = $id++;
        $this->code = $code;
    }

    function getId() {
        return $this->id;
    }

    function process() {
        sleep(3);
        return eval("return {$this->code};");
    }

    function serialize() {
        return serialize([$this->id, $this->code]);
    }

    function unserialize($bin) {
        list($this->id, $this->code) = unserialize($bin);
    }
}

$p = new P7($argv[0], 2);
$p->start();
