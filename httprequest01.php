<?php
$r = new HttpRequest('http://wechat.kaifa.cn/wechatTest/openService/alamService.asmx');
try {
  $r->send();
  echo $r->getResponseBody();
} catch (HttpException $e) {
  echo $e;
}
?>
