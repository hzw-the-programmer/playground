USE TestData
GO

CREATE VIEW vw_Names
AS
SELECT ProductName, Price FROM Products;
GO

SELECT * FROM vw_Names;
GO