mysql -u root -p
show databases;
use mysql;
show tables;
select host, user from user;
show grants for 'pageask'@'localhost'
quit;

mysql -u pageask -p
select host, user from mysql.user;
create user 'hzw'@'localhost' identified by '123456';
show grants for 'hzw'@'localhost';
grant create on iot0.* to 'hzw'@'localhost';
grant drop on iot0.* to 'hzw'@'localhost';
grant references on iot0.* to 'hzw'@'localhost';
grant select on iot0.* to 'hzw'@'localhost';
grant insert on iot0.* to 'hzw'@'localhost';
grant update on iot0.* to 'hzw'@'localhost';
grant delete on iot0.* to 'hzw'@'localhost';
revoke drop on iot0.* from 'hzw'@'localhost';
drop user 'hzw'@'localhost';
show create database iot0;
-- CREATE DATABASE `iot0` /*!40100 DEFAULT CHARACTER SET latin1 */
show create table iot0.device;
-- CREATE TABLE `device` (
--   `id` int(11) NOT NULL AUTO_INCREMENT,
--   `ip` varchar(50) NOT NULL,
--   `port` int(11) NOT NULL,
--   PRIMARY KEY (`id`)
-- ) ENGINE=InnoDB DEFAULT CHARSET=latin1
show create table iot0.channel;
-- CREATE TABLE `channel` (
--   `id` int(11) NOT NULL AUTO_INCREMENT,
--   `did` int(11) NOT NULL,
--   `slot` int(11) NOT NULL,
--   `port` int(11) NOT NULL,
--   PRIMARY KEY (`id`),
--   KEY `fk_did` (`did`),
--   CONSTRAINT `channel_ibfk_1` FOREIGN KEY (`did`) REFERENCES `device` (`id`)
-- ) ENGINE=InnoDB DEFAULT CHARSET=latin1
show create table iot0.alarm_data;
-- CREATE TABLE `alarm_data` (
--   `id` int(11) NOT NULL AUTO_INCREMENT,
--   `cid` int(11) NOT NULL,
--   `channel_time` datetime NOT NULL,
--   `data` int(11) NOT NULL,
--   PRIMARY KEY (`id`),
--   KEY `fk_cid` (`cid`),
--   CONSTRAINT `alarm_data_ibfk_1` FOREIGN KEY (`cid`) REFERENCES `channel` (`id`)
-- ) ENGINE=InnoDB DEFAULT CHARSET=latin1
quit;

mysql -u hzw -p
create database iot0;
drop database iot0;
