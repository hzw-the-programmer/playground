<?php
echo '"" is valid: ' . (filter_var('', FILTER_VALIDATE_IP) ? 'yes' : 'no') . "\n";
echo '0.0.0.0 is valid: ' . (filter_var('0.0.0.0', FILTER_VALIDATE_IP) ? 'yes' : 'no') . "\n";
echo '127.0.0.1 is valid: ' . (filter_var('127.0.0.1', FILTER_VALIDATE_IP) ? 'yes' : 'no') . "\n";
echo '10.0.37.86 is valid: ' . (filter_var('10.0.37.86', FILTER_VALIDATE_IP) ? 'yes' : 'no') . "\n";
