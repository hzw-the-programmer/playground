sudo apt-get update
sudo apt-get install postgresql postgresql-contrib

sudo -i -u postgres
psql

sudo -u postgres psql

\d
\du

CREATE ROLE demo_role;
sudo -u postgres createuser test_user

DROP ROLE demo_role;
DROP ROLE IF EXISTS demo_role;

\h CREATE ROLE

CREATE ROLE demo_role WITH LOGIN;
CREATE USER demo_role;

ALTER ROLE demo_role WITH NOLOGIN;

\password test_user
psql -U test_user -d postgres -h "127.0.0.1" -W

CREATE TABLE demo (
    id serial,
    name varchar(25),
    start_date date
);

\password demo_role
psql -U demo_role -d postgres -h "127.0.0.1" -W
SELECT * FROM demo;
GRANT SELECT ON demo TO demo_role;
REVOKE SELECT ON demo FROM demo_role;

CREATE ROLE temporary_users;
GRANT temporary_users TO demo_role;
GRANT temporary_users TO test_user;

SET ROLE temporary_users;
CREATE TABLE hello (
    id serial,
    name varchar(25),
    start_date date
);

RESET ROLE;
CREATE TABLE world (
    id serial,
    name varchar(25),
    start_date date
);

CREATE TABLE hzw(id serial);
SET ROLE temporary_users;
CREATE TABLE hzw1(id serial);
RESET ROLE;
CREATE TABLE hzw2(id serial);

DROP ROLE temporary_users;
ALTER TABLE hzw1 OWNER TO demo_role;
ALTER TABLE hello OWNER TO demo_role;
DROP ROLE temporary_users;

SELECT datname FROM pg_database;
\l

sudo -u postgres psql
CREATE ROLE loraserver_ns;
ALTER ROLE loraserver_ns LOGIN;
ALTER ROLE loraserver_ns CREATEDB;
\password loraserver_ns
SET ROLE loraserver_ns;
CREATE DATABASE loraserver_ns;

show data_directory;
