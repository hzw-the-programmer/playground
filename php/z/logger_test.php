<?php
include_once "./logger.php";

function test() {
    $data = "\x50\x33" .
            "\x00\x23" .
            "\x03\xF6\x61\xFE\x01\x48\x00\x00" .
            "\x00\x00\x04\x4F" .
            "\x12\x03\x08\x10\x1C\x14" .
            "\x41" .
            "\x02" .
            "\x04" .
            "\x00" .
            "\x09" .
            "\xAB\xCE\x18\x45" .
            "\x04" .
            "\x01" .
            "\x09" .
            "\x11\x01\x19\x45" .
            "\x5F";
    $logger = new Logger();
    $logger->log(time(), "R", ["address" => "192.168.1.23", "port" => 19268], $data);
}

test();
