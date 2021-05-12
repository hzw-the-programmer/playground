<?php
$sql = '
SELECT
p0.id AS p0id, p0.name AS p0name, p0.pid AS p0pid, p0.level AS p0level,
p1.id AS p1id, p1.name AS p1name, p1.pid AS p1pid, p1.level AS p1level,
p2.id AS p2id, p2.name AS p2name, p2.pid AS p2pid, p2.level AS p2level,
p3.id AS p3id, p3.name AS p3name, p3.pid AS p3pid, p3.level AS p3level,
p4.id AS p4id, p4.name AS p4name, p4.pid AS p4pid, p4.level AS p4level,
p5.id AS p5id, p5.name AS p5name, p5.pid AS p5pid, p5.level AS p5level,
p6.id AS p6id, p6.name AS p6name, p6.pid AS p6pid, p6.level AS p6level
FROM places AS p0
LEFT JOIN places AS p1
ON p0.id = p1.pid
LEFT JOIN places AS p2
ON p1.id = p2.pid
LEFT JOIN places AS p3
ON p2.id = p3.pid
LEFT JOIN places AS p4
ON p3.id = p4.pid
LEFT JOIN places AS p5
ON p4.id = p5.pid
LEFT JOIN places AS p6
ON p5.id = p6.pid
WHERE p0.id = ?
ORDER BY p0.name, p1.name, p2.name, p3.name, p4.name, p5.name, p6.name';
$id = isset($_GET['id']) ? $_GET['id'] : '';
if (!is_numeric($id)) {
  $id = 1;
}
$params = [$id];

try {
  include '../db_config.php';
  $dbh = new PDO('sqlsrv:Server=' . $db_config['server_name'] . ';Database=' . $db_config['db_name'],
    $db_config['user_name'], $db_config['password']);

  $places = [];
  
  $stmt = $dbh->prepare($sql);
  if ($stmt->execute($params)) {
    include '../Place.php';
    while ($row = $stmt->fetch(PDO::FETCH_ASSOC)) {
      $place0 = getPlace($row, 0);
      if (is_null($place0->id)) continue;
      $place = addPlace($places, $place0);
    
      $place1 = getPlace($row, 1);
      if (is_null($place1->id)) continue;
      $place = addPlace($place->children, $place1);
    
      $place2 = getPlace($row, 2);
      if (is_null($place2->id)) continue;
      $place = addPlace($place->children, $place2);
    
      $place3 = getPlace($row, 3);
      if (is_null($place3->id)) continue;
      $place = addPlace($place->children, $place3);
    
      $place4 = getPlace($row, 4);
      if (is_null($place4->id)) continue;
      $place = addPlace($place->children, $place4);
    
      $place5 = getPlace($row, 5);
      if (is_null($place5->id)) continue;
      $place = addPlace($place->children, $place5);
    
      $place6 = getPlace($row, 6);
      if (is_null($place6->id)) continue;
      $place = addPlace($place->children, $place6);
    }
  }

  header('Access-Control-Allow-Origin: http://localhost:3000');
  header('Content-type: application/json');
  echo json_encode($places[0], JSON_PRETTY_PRINT | JSON_UNESCAPED_UNICODE);

} catch (PDOException $e) {
  print($e->getMessage());
}
