<?php
//echo 'before session start<br/>';
//echo '$_SESSION: ' . arrayToStr($_SESSION) . '<br/>';
//echo 'SID: ' . SID . '<br/>';
session_start();
echo 'after session start<br/>';
echo '$_SESSION: ' . arrayToStr($_SESSION) . '<br/>';
echo 'SID: ' . SID . '<br/>';

$_SESSION['name'] = 'hzw';
$_SESSION['age'] = 31;
if (array_key_exists('count', $_SESSION)) {
    $_SESSION['count']++;
} else {
    $_SESSION['count'] = 0;
}

function arrayToStr($array) {
    return trim(array_reduce(array_keys($array), function($carry, $key) use ($array) {
        return $carry .= "$key:{$array[$key]}, ";
    }, ''), ' ,');
}
