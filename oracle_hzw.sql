drop user hzw cascade;
create user hzw identified by hzw;
create table hzw.users (
  id number(10) not null,
  firstname varchar2(50),
  lastname varchar2(50) not null,
  phone varchar2(15),
  email varchar2(50),
  constraint users_pk primary key (id)
);
create sequence hzw.users_seq start with 1;
grant unlimited tablespace to hzw;
insert into hzw.users values (hzw.users_seq.nextval, 'zhiwen', 'he', '18676797056', '18676797056@163.com');

grant create session to hzw;
