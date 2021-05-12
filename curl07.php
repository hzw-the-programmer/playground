<?php
    $curl = curl_init();
    // curl_setopt ($curl, CURLOPT_URL, "http://wechat.kaifa.cn/wechatTest/openService/alamService.asmx");
    curl_setopt ($curl, CURLOPT_URL, "http://wechat.kaifa.cn/wechatTest/openService/alamService.asmx?WSDL");
    curl_setopt($curl, CURLOPT_RETURNTRANSFER, 1);

    $result = curl_exec($curl);
    echo curl_error($curl);
    curl_close($curl);
    header('Content-Type: text/html; charset=utf-8');
    print $result;
?>
