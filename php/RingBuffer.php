<?php
class RingBuffer {
    const OVERFLOW_THROW = 'Buffer overflow!';

    const ON_OVERFLOW_THROW = 1;
    const ON_OVERFLOW_DROP = 2;
    const ON_OVERFLOW_SLIDE = 3;
    const ON_OVERFLOW_EXPAND = 4;
    
    private $limit;
    private $length;
    private $pushIndex;
    private $popIndex;
    private $overflowAction;

    private $arr;

    function __construct($limit = 10, $overflowAction) {
        $this->limit = $limit;
        $this->length = 0;
        $this->pushIndex = 0;
        $this->popIndex = 0;
        $this->overflowAction = $overflowAction;

        $this->arr = [];
    }

    private function push($it) {
        $this->arr[$this->pushIndex] = $it;
        $this->pushIndex = ($this->pushIndex + 1) % $this->limit;

        $this->length++;
    }

    function put($it) {
        if ($this->length < $this->limit) {
            $this->push($it);
        } else {
            switch ($this->overflowAction) {
                case RingBuffer::ON_OVERFLOW_THROW:
                    throw new Exception(RingBuffer::OVERFLOW_THROW);
                case RingBuffer::ON_OVERFLOW_SLIDE:
                    $this->arr[$this->pushIndex] = $it;
                    $this->pushIndex = ($this->pushIndex + 1) % $this->limit;
                    $this->popIindex = $this->pushIndex;
                    break;
                case RingBuffer::ON_OVERFLOW_EXPAND:
                    $this->arr = $this->flush();

                    $this->length = $this->limit;
                    $this->limit *= 2;
                    $this->pushIndex = $this->length;
                    $this->popIndex = 0;

                    $this->push($it);
                    break;
                default:
                    // DROP
            }
        }
    }

    function take() {
        if ($this->length > 0) {
            $it = $this->arr[$this->popIndex];
            $this->popIndex = ($this->popIndex + 1) % $this->limit;

            $this->length--;

            return $it;
        }
    }

    function isEmpty() {
        return $this->length === 0;
    }

    function flush() {
        $items = [];
        while ($this->length) {
            $items[] = $this->take();
        }
        return $items;
    }

    static function none() {
        return new RingBuffer(0, RingBuffer::ON_OVERFLOW_THROW);
    }

    static function fixed($limit) {
        return new RingBuffer($limit, RingBuffer::ON_OVERFLOW_THROW);
    }

    static function dropping($limit) {
        return new RingBufer($limit, RingBuffer::ON_OVERFLOW_DROP);
    }

    static function sliding($limit) {
        return new RingBuffer($limit, RingBuffer::ON_OVERFLOW_SLIDE);
    }

    static function expanding($initialSize) {
        return new RingBuffer($initialSize, RingBuffer::ON_OVERFLOW_EXPAND);
    }
}


