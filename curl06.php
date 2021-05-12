<?php
    $curl = curl_init();
    curl_setopt($curl, CURLOPT_URL,"http://localhost:8000/posttest.php");
    curl_setopt($curl, CURLOPT_POST, 1);
    curl_setopt($curl, CURLOPT_POSTFIELDS, "Hello=World&Foo=Bar&Baz=Wombat");

    curl_exec ($curl);
    curl_close ($curl);
?>
