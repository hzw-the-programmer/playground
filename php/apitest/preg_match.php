<?php
//$pattern = '/php/';
$pattern = '/php/i';
$subject = 'PHP is the web scripting language of choice. I love php.';
//if (preg_match($pattern, $subject)) {
//if (preg_match($pattern, $subject, $matches)) {
//if (preg_match($pattern, $subject, $matches, PREG_OFFSET_CAPTURE)) {
//if (preg_match($pattern, $subject, $matches, PREG_OFFSET_CAPTURE, 1)) {
//if (preg_match_all($pattern, $subject, $matches)) {
if (preg_match_all($pattern, $subject, $matches, PREG_SET_ORDER)) {
    //echo "matched\n";
    var_dump($matches);
} else {
    echo "unmatched\n";
}
