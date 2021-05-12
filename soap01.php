<?
header('Content-Type: text/html; charset=utf-8');

$receiveUser = array();
$receiveUser[0] = ['empCode' => '01074347'];
// $receiveUser[1] = ['empCode' => '01073615'];
// $receiveUser[2] = ['empCode' => '01073613'];
// $receiveUser[3] = ['empCode' => '01073616'];

$params = array();
$params['alarmPosition'] = '客厅门磁';
$params['alamContent'] = '客厅门磁发生报警';
$params['alarmTime'] = '2017年06月20日 11:12';
$params['receiveUser'] = $receiveUser;
// $params = 'alamContent=' . json_encode($params);
// $params = json_encode($params);
$params = 'Hi QiuFengNie';
echo $params;
echo '<br>';

$client = new soapclient(
  'http://wechat.kaifa.cn/wechatTest/openService/alamService.asmx?WSDL',
  array('trace' => 1));
$result = $client->KedasAlam($params);
echo "Request Headers:<br>" . $client->__getLastRequestHeaders() . "<br>";
echo "Request:<br>" . $client->__getLastRequest() . "<br>";
echo "Response:<br>" . $client->__getLastResponse() . "<br>";
?>
