mkdir play
cd play
go mod init hzw.org/play

go env -w GOPROXY="https://goproxy.cn"
go install github.com/spf13/cobra-cli@latest
cobra-cli init

mysql -u root -p
select host, user from mysql.user;
ALTER USER 'root'@'%' IDENTIFIED BY 'RooT@123';
show database;
show tables;
create database gorm;
grant all privileges on gorm.* to 'root'@'%';
