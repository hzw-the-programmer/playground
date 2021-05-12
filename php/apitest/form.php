<?php
function kvstr($arr) {
    return trim(array_reduce(
        array_keys($arr),
        function ($carry, $key) use ($arr) {
            $value = $arr[$key];
            return $carry . "{$key}={$value},";
        },
        ''
    ), ',');
}

echo '$_POST:' . kvstr($_POST) . '<br/>';
echo '$_GET:' . kvstr($_GET) . '<br/>';
$content = file_get_contents('php://input');
echo "content1: $content<br/>";
$content = file_get_contents('php://input');
echo "content2: $content<br/>";
