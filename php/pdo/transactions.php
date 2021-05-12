<?php
/*
1. make conn verytime:
    cost: 31.639842987061
    cost: 31.976830005646
    cost: 31.117220163345
    cost: 34.89594912529
    cost: 31.625918149948
    cost: 31.929342031479
    cost: 31.517198085785
    cost: 31.801046848297
    cost: 31.072865962982
    cost: 31.382237911224
2. make conn onlyonce:
    cost: 9.6693398952484
    cost: 9.1532049179077
    cost: 12.470614910126
    cost: 8.6154720783234
    cost: 8.8884701728821
    cost: 9.6282179355621
    cost: 10.080868959427
    cost: 7.8944931030273
    cost: 10.198842048645
    cost: 9.8412420749664
*/

if ($argc !== 4) {
    die("{$argv[0]} ip user passwd\n");
}

$dsn = "sqlsrv:Server={$argv[1]};Database=test";
$user = $argv[2];
$passwd = $argv[3];

$start = microtime(true);
$dbh = new PDO($dsn, $user, $passwd);
$stmt = $dbh->prepare('INSERT INTO test (num) VALUES (:num)');
for ($i = 0; $i < 100; $i++) {
    try {
        //$dbh = new PDO($dsn, $user, $passwd);
        //$stmt = $dbh->prepare('INSERT INTO test (num) VALUES (:num)');
    
        for ($j = 0; $j < 10; $j++) {
            $num = $j;
            
            $dbh->beginTransaction();

            $stmt->bindParam(':num', $num);
            $stmt->execute();
            
            if ($j % 2 === 0) {
                $dbh->rollBack();
            } else {
                $dbh->commit();
            }
        }
    } catch (PDOException $e) {
        echo $e->getMessage() . "\n";
    } finally {
        //$dbh = null;
    }    
}
echo "cost: " . (microtime(true) - $start) . "\n";
