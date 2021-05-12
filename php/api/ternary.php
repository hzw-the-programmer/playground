<?php
function status($status) {
    /*
    if ($status & 0x80) return 0x80;
    if (($status & 0x50) === 0x50) return 0x50;
    if ($status & 0x40) return 0x40;
    return $status & 0x20;
    */

    return $status & 0x80 ? 0x80 : (/*need parentheses here*/
        ($status & 0x50)/*need parentheses here*/ === 0x50 ? 0x50 : (/*need parentheses here*/
        $status & 0x40 ? 0x40 : $status & 0x20
    ));
}

echo true ? 'true' : false ? 't' : 'f';
echo "\n";
echo (true ? 'true' : false) ? 't' : 'f';
echo "\n";
echo true ? 'true' : (false ? 't' : 'f');
echo "\n";
echo false ? 'true' : (false ? 't' : 'f');
echo "\n";

echo sprintf('%02x', status(0x80)) . "\n";
echo sprintf('%02x', status(0x50)) . "\n";
echo sprintf('%02x', status(0x40)) . "\n";
echo sprintf('%02x', status(0x20)) . "\n";
echo sprintf('%02x', status(0x18)) . "\n";
