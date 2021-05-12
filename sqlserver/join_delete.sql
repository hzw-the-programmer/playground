DELETE rs
FROM mpoint AS mp
LEFT JOIN realtime_status AS rs
ON mp.ciid = rs.ciid
WHERE mp.pid != rs.pid
