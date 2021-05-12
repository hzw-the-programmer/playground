SELECT
b.sn, b.slot, b.port, b.devicetype,
c.plant, c.workshop, c.region, c.line, c.station, c.channel,
a.value, TO_CHAR(a.time, 'YYYY-MM-DD HH24:MI:SS') time,
case
  when a.value != LAG(a.value)
    rownum
  else
    null
  end grp
FROM ddata a, device b, place c
WHERE a.did = b.did AND a.pid = c.pid AND
b.sn LIKE '%' AND b.slot LIKE '%' AND b.port LIKE '%' AND b.devicetype LIKE '%' AND
c.plant LIKE '%' AND c.workshop LIKE '%' AND c.region LIKE '%' AND c.line LIKE '%' AND c.station LIKE '%' AND c.channel LIKE '%' AND
a.time BETWEEN TO_DATE('19870123', 'YYYYMMDD') AND TO_DATE('20500123', 'YYYYMMDD')
ORDER BY b.sn, b.slot, b.port, b.devicetype, a.time DESC;

SELECT
  did,
  pid,
  TO_CHAR(time, 'YYYY-MM-DD HH24:MI:SS') starttime,
  value,
  CASE WHEN did != LAG(did, 1, -1) OVER (ORDER BY did, pid, time) OR
            pid != LAG(pid, 1, -1) OVER (ORDER BY did, pid, time) OR
            value != LAG(value, 1, -1) OVER (ORDER BY did, pid, time)
  THEN 1
  ELSE 0 END first
FROM ddata
ORDER BY did, pid, time

SELECT * FROM (
SELECT
  did,
  pid,
  TO_CHAR(time, 'YYYY-MM-DD HH24:MI:SS') starttime,
  value,
  CASE WHEN did != LAG(did, 1, -1) OVER (ORDER BY did, pid, time) OR
            pid != LAG(pid, 1, -1) OVER (ORDER BY did, pid, time) OR
            value != LAG(value, 1, -1) OVER (ORDER BY did, pid, time)
  THEN 1
  ELSE 0 END first
FROM ddata
ORDER BY did, pid, time)
WHERE first = 1

SELECT
  did,
  pid,
  TO_CHAR(time, 'YYYY-MM-DD HH24:MI:SS') starttime,
  value
FROM ddata

SELECT
  did,
  pid,
  TO_CHAR(time, 'YYYY-MM-DD HH24:MI:SS') starttime,
  value,
  ROW_NUMBER() OVER (PARTITION BY did, pid ORDER BY time) id
FROM ddata

SELECT
  did,
  pid,
  COUNT(*)
FROM ddata
GROUP BY did, pid
ORDER BY did, pid

SELECT
  did,
  pid,
  time,
  value,
  id,
  id - ROW_NUMBER() OVER (PARTITION BY did, pid, value ORDER BY time)
FROM (
  SELECT
    did,
    pid,
    time,
    value,
    ROW_NUMBER() OVER (PARTITION BY did, pid ORDER BY time) id
  FROM ddata)






SELECT
  did,
  pid,
  value,
  TO_CHAR(MIN(time), 'YYYY-MM-DD HH24:MI:SS') starttime,
  TO_CHAR(MAX(time), 'YYYY-MM-DD HH24:MI:SS') endtime,
  COUNT(*)
FROM (
  SELECT
    did,
    pid,
    time,
    value,
    id,
    id - ROW_NUMBER() OVER (PARTITION BY did, pid, value ORDER BY time) grp
  FROM (
    SELECT
      did,
      pid,
      time,
      value,
      ROW_NUMBER() OVER (PARTITION BY did, pid ORDER BY time) id
    FROM ddata
  )
)
GROUP BY did, pid, value, grp
ORDER BY did, pid, starttime


SELECT
  did,
  pid,
  value,
  TO_CHAR(MIN(time), 'YYYY-MM-DD HH24:MI:SS') starttime,
  TO_CHAR(MAX(time), 'YYYY-MM-DD HH24:MI:SS') endtime,
  COUNT(*) total
FROM (
  SELECT
    did,
    pid,
    time,
    value,
    id,
    id - ROW_NUMBER() OVER (PARTITION BY did, pid, value ORDER BY time) grp
  FROM (
    SELECT
      did,
      pid,
      time,
      value,
      ROW_NUMBER() OVER (PARTITION BY did, pid ORDER BY time) id
    FROM ddata
  )
)
GROUP BY did, pid, value, grp
ORDER BY did, pid, starttime



SELECT b.sn, b.slot, b.port, b.devicetype, c.plant, c.workshop, c.region, c.line, c.station, c.channel, a.starttime, a.endtime, a.total, a.value
FROM (
  SELECT
    did,
    pid,
    value,
    TO_CHAR(MIN(time), 'YYYY-MM-DD HH24:MI:SS') starttime,
    TO_CHAR(MAX(time), 'YYYY-MM-DD HH24:MI:SS') endtime,
    COUNT(*) total
  FROM (
    SELECT
      did,
      pid,
      time,
      value,
      id,
      id - ROW_NUMBER() OVER (PARTITION BY did, pid, value ORDER BY time) grp
    FROM (
      SELECT
        did,
        pid,
        time,
        value,
        ROW_NUMBER() OVER (PARTITION BY did, pid ORDER BY time) id
      FROM ddata
    )
  )
  GROUP BY did, pid, value, grp
  ORDER BY did, pid, starttime
) a, device b, place c
WHERE a.did = b.did AND a.pid = c.pid
ORDER BY b.sn, b.slot, b.port, b.devicetype, a.starttime

SELECT * FROM (SELECT X.*, ROWNUM RN FROM (SELECT b.sn, b.slot, b.port, b.devicetype, c.plant, c.workshop, c.region, c.line, c.station, c.channel, a.starttime, a.endtime, a.total, a.value FROM ( SELECT did, pid, value, TO_CHAR(MIN(time), 'YYYY-MM-DD HH24:MI:SS') starttime, TO_CHAR(MAX(time), 'YYYY-MM-DD HH24:MI:SS') endtime, COUNT(*) total FROM ( SELECT did, pid, time, value, id, id - ROW_NUMBER() OVER (PARTITION BY did, pid, value ORDER BY time) grp FROM ( SELECT did, pid, time, value, ROW_NUMBER() OVER (PARTITION BY did, pid ORDER BY time) id FROM ddata ) ) GROUP BY did, pid, value, grp ORDER BY did, pid, starttime ) a, device b, place c WHERE a.did = b.did AND a.pid = c.pid AND b.sn LIKE '%' AND b.slot LIKE '%' AND b.port LIKE '%' AND c.plant LIKE '%' AND c.workshop LIKE '%' AND c.region LIKE '%' AND c.line LIKE '%' AND c.station LIKE '%' AND c.channel LIKE '%' AND a.starttime >= TO_DATE('2000-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS') AND a.endtime <= TO_DATE('2999-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS') ORDER BY b.sn, b.slot, b.port, b.devicetype, a.starttime) X WHERE ROWNUM <= 10) WHERE RN > 0



***************************************************************************
SELECT
  did,
  pid,
  value,
  MIN(time) starttime,
  MAX(time) endtime,
  COUNT(*) total,
  grp
FROM (
  SELECT
    did,
    pid,
    time,
    value,
    gid,
    gid - ROW_NUMBER() OVER (PARTITION BY did, pid, value ORDER BY gid) grp
  FROM (
    SELECT
      did,
      pid,
      time,
      value,
      ROW_NUMBER() OVER (PARTITION BY did, pid ORDER BY time, id) gid
    FROM
      ddata
    WHERE
      time >= TO_DATE('1904-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS') AND
      time <= TO_DATE('1904-01-02 00:00:00', 'YYYY-MM-DD HH24:MI:SS')
  )
)
GROUP BY did, pid, value, grp
ORDER BY did, pid, starttime, grp
***************************************************************************
