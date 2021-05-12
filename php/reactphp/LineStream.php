<?php
use Evenement\EventEmitter;
use React\Stream\ReadableResourceStream;

class LineStream extends EventEmitter {
    private $stream;
    private $data = '';

    public function __construct($streamResouce, $loop) {
        $this->stream = new ReadableResourceStream($streamResouce, $loop, 1);
        $this->stream->on('data', [$this, 'handleData']);
    }

    public function handleData($data) {
        $this->data .= $data;
        while (($pos = strpos($this->data, "\n")) !== false) {
            $this->emit('line', [substr($this->data, 0, $pos)]);
            $this->data = substr($this->data, $pos + 1);
        }
    }
}
