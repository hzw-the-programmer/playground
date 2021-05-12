clear screen
accept uname prompt 'Enter User Name : '
accept outfile prompt  ' Output filename : '
 
spool &&outfile..gen
 
SET LONG 20000 LONGCHUNKSIZE 20000 PAGESIZE 0 LINESIZE 1000 FEEDBACK OFF VERIFY OFF TRIMSPOOL ON
 
BEGIN
   DBMS_METADATA.set_transform_param (DBMS_METADATA.session_transform, 'SQLTERMINATOR', true);
   DBMS_METADATA.set_transform_param (DBMS_METADATA.session_transform, 'PRETTY', true);
END;
/
 
SELECT dbms_metadata.get_ddl('USER','&&uname') FROM dual;
SELECT DBMS_METADATA.GET_GRANTED_DDL('SYSTEM_GRANT','&&uname') from dual;
SELECT DBMS_METADATA.GET_GRANTED_DDL('ROLE_GRANT','&&uname') from dual;
SELECT DBMS_METADATA.GET_GRANTED_DDL('OBJECT_GRANT','&&uname') from dual;
 
spool off