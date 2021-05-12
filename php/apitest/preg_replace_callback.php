<?php
$text = "April fools day is 04/01/2019\n";
$text .= "Last christmas was 12/24/2017\n";

$text = preg_replace_callback(
    '|(\d{2}/\d{2}/)(\d{4})|',
    function ($matches) {
        return $matches[1] . ($matches[2] + 1);
    },
    $text
);

echo $text;
