sudo apt update
sudo apt install mysql-server
systemctl status mysql.service
mysql_secure_installation
mysqladmin -p -u root version

mysql -u root -p
CREATE USER 'hzw'@'localhost' IDENTIFIED BY 'password';
ALTER USER 'hzw'@'localhost' IDENTIFIED BY 'new_password';
//enable create database
GRANT CREATE ON website_skeleton.* TO 'hzw'@'localhost';
