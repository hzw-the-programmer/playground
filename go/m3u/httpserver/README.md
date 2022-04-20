https://www.digitalocean.com/community/tutorials/how-to-install-mysql-on-ubuntu-20-04
https://www.digitalocean.com/community/tutorials/how-to-allow-remote-access-to-mysql

### install
sudo apt update
sudo apt install mysql-server
// sudo systemctl start mysql.service
sudo mysql_secure_installation

### root login
sudo mysql
CREATE USER 'gorm'@'_gateway' IDENTIFIED BY 'gorM@123';
grant create, select, insert, delete on gorm.* to 'gorm'@'_gateway';
// create database gorm;
quit
mysql -u gorm -h _gateway -p
create database gorm;

### check mysql listen address
netstat -tlnp
tcp        0      0 127.0.0.1:3306          0.0.0.0:*               LISTEN      -
sudo vi /etc/mysql/mysql.conf.d/mysqld.cnf
bind-address            = 127.0.0.1
bind-address            = *
sudo systemctl restart mysql
netstat -tlnp
tcp6       0      0 :::3306                 :::*                    LISTEN      -

### other staff
sudo apt update
sudo apt install mysql-server
sudo systemctl start mysql.service
sudo mysql_secure_installation

```
Note that even though you’ve set a password for the root MySQL user, this user is not currently configured to authenticate with a password when connecting to the MySQL shell.
In Ubuntu systems running MySQL 5.7 (and later versions), the root MySQL user is set to authenticate using the auth_socket plugin by default rather than with a password. This plugin requires that the name of the operating system user that invokes the MySQL client matches the name of the MySQL user specified in the command, so you must invoke mysql with sudo privileges to gain access to the root MySQL user:
```
sudo mysql

```
If you installed MySQL with another tutorial and enabled password authentication for root, you will need to use a different command to access the MySQL shell. 
```
// mysql -u root -p

// CREATE USER 'username'@'host' IDENTIFIED WITH authentication_plugin BY 'password';
// caching_sha2_password
CREATE USER 'sammy'@'localhost' IDENTIFIED BY 'password';
// CREATE USER 'sammy'@'localhost' IDENTIFIED WITH mysql_native_password BY 'password';
// ALTER USER 'sammy'@'localhost' IDENTIFIED WITH mysql_native_password BY 'password';

mysql -u gorm -p

// GRANT PRIVILEGE ON database.table TO 'username'@'host';
// GRANT CREATE, ALTER, DROP, INSERT, UPDATE, DELETE, SELECT, REFERENCES, RELOAD on *.* TO 'sammy'@'localhost' WITH GRANT OPTION;
// GRANT ALL PRIVILEGES ON *.* TO 'sammy'@'localhost' WITH GRANT OPTION;

FLUSH PRIVILEGES;

exit

``
mysql -u gorm -p
create database gorm;
show databases;
quit
sudo mysql
grant create on gorm.* to 'gorm'@'localhost';
quit
mysql -u gorm -p
create database gorm;
create database gorm1;
show databases;
``

```
create user 'gorm'@'%' IDENTIFIED BY 'gorM@123';
select Host, User from mysql.user;
```

netstat -tlnp
tcp        0      0 127.0.0.1:3306          0.0.0.0:*               LISTEN      -
sudo vi /etc/mysql/mysql.conf.d/mysqld.cnf
bind-address            = 127.0.0.1
bind-address            = *
sudo systemctl restart mysql
netstat -tlnp
tcp6       0      0 :::3306                 :::*                    LISTEN      -

select Host, User from mysql.user;
RENAME USER 'sammy'@'localhost' TO 'sammy'@'remote_server_ip';
select Host, User from mysql.user;
grant create on gorm1.* to 'gorm1'@'_gateway';
mysql -u gorm1 -h _gateway -p
create database gorm1;
