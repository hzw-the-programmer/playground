<?php
/*
function once($cb) {
    $called = false;
    return function() use ($cb, &$called) {
        if (!$called) {
            $called = true;
            $cb();
        }
    };
}
*/

class State {
    function __construct() {
        $this->called = false;
    }

    function __destruct() {
        echo "destruct\n";
    }
}

function once($cb) {
    $state = new State;
    return function() use ($cb, $state) {
        if (!$state->called) {
            $state->called = true;
            $cb();
        }
    };
}

$func = once(function() {
    echo "func1 called\n";
});
echo "first call\n";
$func();
echo "second call\n";
$func();

$func = once(function() {
    echo "func2 called\n";
});
echo "first call\n";
$func();
echo "second call\n";
$func();

echo "finish\n";
