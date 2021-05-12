https://stackoverflow.com/questions/34377338/linux-php-7-0-and-mssql-microsoft-sql

Ubuntu 16.04:
sudo su 
curl https://packages.microsoft.com/keys/microsoft.asc | apt-key add -
curl https://packages.microsoft.com/config/ubuntu/16.04/prod.list > /etc/apt/sources.list.d/mssql-release.list
exit

sudo apt update
sudo ACCEPT_EULA=Y apt install -y msodbcsql mssql-tools unixodbc-dev
sudo pecl install sqlsrv
sudo pecl install pdo_sqlsrv

//echo "extension=sqlsrv" >> `php --ini | grep "Loaded Configuration" | sed -e "s|.*:\s*||"`
//echo "extension=pdo_sqlsrv" >> `php --ini | grep "Loaded Configuration" | sed -e "s|.*:\s*||"`

echo "extension=sqlsrv.so" >> `php --ini | grep "Loaded Configuration" | sed -e "s|.*:\s*||"`
//echo "extension=pdo_sqlsrv.so" >> `php --ini | grep "Loaded Configuration" | sed -e "s|.*:\s*||"`
echo "extension=pdo_sqlsrv.so" >> /etc/php/7.0/mods-available/pdo.ini

CentOS 7:
sudo su
curl https://packages.microsoft.com/config/rhel/7/prod.repo > /etc/yum.repos.d/mssql-release.repo
exit

sudo yum update
sudo ACCEPT_EULA=Y yum install -y msodbcsql mssql-tools unixODBC-devel 
sudo yum groupinstall "Development Tools"
sudo pecl install sqlsrv
sudo pecl install pdo_sqlsrv

echo "extension=sqlsrv" >> `php --ini | grep "Loaded Configuration" | sed -e "s|.*:\s*||"`
echo "extension=pdo_sqlsrv" >> `php --ini | grep "Loaded Configuration" | sed -e "s|.*:\s*||"`

Verify sqlsrv:
<?php
$serverName = "192.168.1.123";
$connectionOptions = array(
    "Database" => "zserver",
    "Uid" => "sa",
    "PWD" => "hzw"
);
//Establishes the connection
$conn = sqlsrv_connect($serverName, $connectionOptions);
if($conn)
    echo "Connected!\n";
else
    print_r(sqlsrv_errors());
?>

Verify pdo_sqlsrv:
<?php
$dsn = "sqlsrv:Server=192.168.1.123;Database=zserver";
$user = "sa";
$password = "hzw";

try {
    $dbh = new PDO($dsn, $user, $password);
    echo "Connected!\n";
} catch (PDOException $e) {
    echo 'Connection failed: ' . $e->getMessage() . "\n";
}
undefined symbol: php_pdo_register_driver in Unknown on line 0
