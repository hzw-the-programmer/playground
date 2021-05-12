SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[StationsUnderPlace]
	@pid bigint
AS
BEGIN
	SET NOCOUNT ON;

	SELECT p5.id, p5.name, p5.pid, p5.level
	FROM places AS p5
	LEFT JOIN places AS p4
	ON p5.pid = p4.id
	LEFT JOIN places AS p3
	ON p4.pid = p3.id
	LEFT JOIN places AS p2
	ON p3.pid = p2.id
	LEFT JOIN places AS p1
	ON p2.pid = p1.id
	LEFT JOIN places AS p0
	ON p1.pid = p0.id
	WHERE p5.level = 5 AND
		  (
			p0.id = @pid OR
			p1.id = @pid OR
			p2.id = @pid OR
			p3.id = @pid OR
			p4.id = @pid
		  )
END
