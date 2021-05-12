sudo vi /etc/mysql/mysql.conf.d/mysqld.cnf
comment out
#bind-address		= 127.0.0.1

sudo systemctl restart mysql

CREATE USER 'root'@'%' IDENTIFIED BY 'asdF@123';
GRANT ALL ON website_skeleton.* TO 'root'@'%';
