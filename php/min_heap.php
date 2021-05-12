<?php
class MinHeap extends SplHeap {
    public function compare($v1, $v2) {
        //return $v1 - $v2;
        return $v2 - $v1;
    }

    public function extractMinItems() {
        $results = [];
        $pre = NULL;
        $cur = NULL;
        while ($this->valid()) {
            $cur = $this->top();
            if ($pre == NULL || $pre == $cur) {
                $results[] = $this->extract();
            } else {
                break;
            }
            $pre = $cur;
        }
        return $results;
    }
}

$mh = new MinHeap();

$mh->insert(10);
$mh->insert(9);
$mh->insert(9);
$mh->insert(9);
$mh->insert(8);

echo $mh->top() . "\n";
echo $mh->top() . "\n";

var_dump($mh->extractMinItems());
var_dump($mh->extractMinItems());
var_dump($mh->extractMinItems());
var_dump($mh->extractMinItems());
