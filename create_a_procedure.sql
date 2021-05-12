USE TestData
GO

CREATE PROCEDURE pr_Names
  @VarPrice money
AS
BEGIN
  -- The print statement returns text to the user
  PRINT 'Products less than ' + CAST(@VarPrice AS varchar(10));
  -- A second statement starts here
  SELECT ProductName, Price
  FROM vw_Names
  WHERE Price < @varPrice;
END
GO

EXECUTE pr_Names 10.00;
GO