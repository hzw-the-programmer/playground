<?php
    $curl = curl_init();
    $fp = fopen("somefile.txt", "w");
    curl_setopt ($curl, CURLOPT_URL, "http://www.php.net");
    curl_setopt($curl, CURLOPT_FILE, $fp);

    curl_exec ($curl);
    curl_close ($curl);
?>
