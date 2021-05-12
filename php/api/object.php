<?php
$json = "{\"sns\":[\"20190123000001\"],\"cmd\":82,\"type\":8}";
$obj = json_decode($json);
var_dump($obj);
//if ($obj->task) {
    if (isset($obj->task)) {
    echo "haha\n";
} else {
    echo "hehe\n";
}
