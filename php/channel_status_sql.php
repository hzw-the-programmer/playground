<?php
$selectCount = '
SELECT COUNT(*)
';

$select = '
SELECT
d.sn, c.slot, c.port, t.name AS type,
p1.name AS plant,
p2.name AS workshop,
p3.name AS region,
p4.name AS line,
p5.name AS station,
p6.name AS channel,
CONVERT(char, cs.ctime, 120) AS stime,
CONVERT(char, cs.endtime, 120) AS etime,
cs.status
';

$starttime = isset($_GET['starttime']) ? $_GET['starttime'] : '';
$endtime = isset($_GET['endtime']) ? $_GET['endtime'] : '';

$from = "
FROM (
  SELECT
  id, cid, pid,
  CASE
    WHEN ctime < '$starttime' THEN '$starttime'
    ELSE ctime
  END AS ctime,
  CASE
    WHEN endtime > '$endtime' THEN '$endtime'
    ELSE endtime
  END AS endtime,
  status, pstatus, nstatus
  FROM
  (
    SELECT
    *,
    LEAD(ctime) OVER (PARTITION BY cid, pid ORDER BY ctime) AS endtime
    FROM
    (
      SELECT
      *,
      LAG(status) OVER (PARTITION BY cid, pid ORDER BY ctime) AS pstatus,
      LEAD(status) OVER (PARTITION BY cid, pid ORDER BY ctime) AS nstatus
      FROM
      (
        SELECT
        id, cid, pid, ctime, status
        FROM (
          SELECT
          *,
          LAG(status) OVER (PARTITION BY cid, pid ORDER BY ctime) AS pstatus,
          LEAD(status) OVER (PARTITION BY cid, pid ORDER BY ctime) AS nstatus
          FROM (
            SELECT
            id, cid, pid, ctime,
            CASE
              WHEN status < 256 THEN status &0xF0
              ELSE status
            END AS status
            FROM channel_status AS cs
          ) AS cs
        ) AS cs
        WHERE status != 259 AND (status != 256 OR nstatus IS NULL)
      ) AS cs
    ) AS cs
    WHERE pstatus IS NULL OR status != pstatus
  ) AS cs
  WHERE ctime BETWEEN '$starttime' AND '$endtime'
    OR endtime BETWEEN '$starttime' AND '$endtime'
    OR ctime < '$starttime' AND (endtime IS NULL OR endtime > '$endtime')
) AS cs
LEFT JOIN channels AS c
ON cs.cid = c.id
LEFT JOIN devices AS d
ON c.did = d.id
LEFT JOIN types AS t
ON c.type = t.id
LEFT JOIN places AS p6
ON cs.pid = p6.id
LEFT JOIN places AS p5
ON p6.pid = p5.id
LEFT JOIN places AS p4
ON p5.pid = p4.id
LEFT JOIN places AS p3
ON p4.pid = p3.id
LEFT JOIN places AS p2
ON p3.pid = p2.id
LEFT JOIN places AS p1
ON p2.pid = p1.id
";

include(dirname(__FILE__) . '/where.php');

$orderBy = '
ORDER BY d.sn, c.slot, c.port, cs.ctime DESC
';

include(dirname(__FILE__) . '/limit.php');

// if ($starttime != '') {
//   $where .= '
// AND cs.ctime >= ?
// ';
//   $params[] = &$starttime;
// }

// if ($endtime != '') {
//   $where .= '
// AND cs.ctime <= ?
// ';
//   $params[] = &$endtime;
// }
