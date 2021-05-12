CREATE TABLE [dbo].[t1](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[did] [int] NOT NULL,
	[time] [int] NOT NULL,
	[status] [int] NOT NULL
) ON [PRIMARY]

DECLARE @st int
DECLARE @et int
SET @st = 0
SET @et = 25

SELECT
did, SUM(duration)
FROM (
SELECT
*,
CASE
	WHEN ltime IS NULL THEN @et - time
	ELSE ltime - time
END AS duration
FROM (
SELECT
did,
CASE
	WHEN ptime IS NULL AND status = 1 THEN 0
	ELSE status
END AS status,
CASE
	WHEN ptime IS NULL AND status = 1 THEN @st
	ELSE time
END AS time,
CASE
	WHEN ptime IS NULL AND status = 1 THEN time
	ELSE ltime
END AS ltime
FROM (
	SELECT
	did,
	status,
	LAG(time) OVER (PARTITION BY did ORDER BY time) AS ptime,
	time,
	LEAD(time) OVER (PARTITION BY did ORDER BY time) AS ltime
	FROM t1
) t1
) AS t1
WHERE status = 0
) AS t2
GROUP BY did