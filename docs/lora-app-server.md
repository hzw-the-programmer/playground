PATH=$PATH:/home/zhiwenhe/work/go/bin:/home/zhiwenhe/go/bin:/home/zhiwenhe/build/protobuf/bin
export GOPROXY="https://goproxy.cn"

make dev-requirements
make ui-requirements
make api
make

sudo -u postgres psql

CREATE ROLE loraserver_as LOGIN CREATEDB;
\password loraserver_as

CREATE ROLE loraserver_as WITH LOGIN PASSWORD 'loraserver_as';

SET ROLE loraserver_as;
CREATE DATABASE loraserver_as;

CREATE DATABASE loraserver_as WITH OWNER loraserver_as;

\c loraserver_as
CREATE EXTENSION pg_trgm;
CREATE EXTENSION hstore;
