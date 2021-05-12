<?php
require './DataManager.php';

$action = $argv[1];
$fn = 'data';

if ($action === 'backup') {
    $sn = $argv[2];
    $count = $argv[3];
    $type = 'ChannelStatus';
    
    DataManager::backup($fn, function($f) use ($sn, $count, $type) {
        for ($i = 0; $i < $count; $i++) {
            for ($slot = 1; $slot < 5; $slot++) {
                for ($port = 0; $port < 4; $port++) {
                    fputcsv($f, [
                        $type, $sn, $slot, $port
                    ]);
                }
            }
            $sn++;
        }
    });    
} else {
    while (true) {
        $continue = DataManager::restore($fn, function($f) {
            $i = 0;
            while ($i < 16 && ($l = fgetcsv($f))) {
                echo implode(',', $l) . "\n";
                $i++;
            }
        }) ? 'true' : 'false';
        //echo "before sleep, continue=$continue\n";
        sleep(10);
    }
}
