<?
$db_config = [
  'server_name' => '127.0.0.1',
  'db_name' => 'v2_2_2',
  'user_name' => 'sa',
  'password' => 'zhiwenhe'
];

$conn = sqlsrv_connect(
  $db_config['server_name'],
  ['Database' => $db_config['db_name'],
   'UID' => $db_config['user_name'],
   'PWD' => $db_config['password'],
   'CharacterSet' => 'UTF-8'
  ]
);

if(!$conn) {
  header('Content-type: text/javascript; charset=utf-8');
  die(print_r(sqlsrv_errors(), true));
}

$pid = isset($_GET['pid']) ? $_GET['pid'] : '';
if (!is_numeric($pid)) {
  $pid = '1';
}

$sql = 'SELECT id, name, pid, level FROM places WHERE pid = ?';
$params = [&$pid];

$stmt = sqlsrv_prepare($conn, $sql, $params);
if (!$stmt) {
  die(print_r(sqlsrv_errors(), true));
}
if (!sqlsrv_execute($stmt)) {
  die(print_r(sqlsrv_errors(), true));
}
$places = [];
while ($row = sqlsrv_fetch_array($stmt, SQLSRV_FETCH_ASSOC)) {
  $places[] = $row;
}
sqlsrv_free_stmt($stmt);

sqlsrv_close($conn);

header('Access-Control-Allow-Origin: http://localhost:8000');
header('Content-type: text/javascript; charset=utf-8');
echo json_encode($places, JSON_PRETTY_PRINT | JSON_UNESCAPED_UNICODE);

class Place {
  public $id;
  public $name;
  public $pid;
  public $level;
  public $children;

  function __construct($id, $name, $pid, $level, $children) {
    $this->id = $id;
    $this->name = $name;
    $this->pid = $pid;
    $this->level = $level;
    $this->children = $children;
  }
}

function getPlace($row, $level) {
  $prefix = 'p' . $level;
  $id = $prefix . 'id';
  $name = $prefix . 'name';
  $pid = $prefix . 'pid';
  $level = $prefix . 'level';

  $place = new Place($row[$id], $row[$name], $row[$pid], $row[$level], []);
  return $place;
}

function findPlace($places, $place) {
  foreach ($places as $p) {
    if ($p->id == $place->id) {
      return $p;
    }
  }
  return null;
}

function addPlace(&$places, $place) {
  $fplace = findPlace($places, $place);
  if (is_null($fplace)) {
    $places[] = $place;
    return $place;
  }
  return $fplace;
}
