CREATE TABLE ddata (
  id NUMBER CONSTRAINT ddata_pk PRIMARY KEY,
  did NUMBER,
  pid NUMBER,
  value NUMBER,
  time DATE
);
CREATE SEQUENCE ddata_seq
  MINVALUE -1;
