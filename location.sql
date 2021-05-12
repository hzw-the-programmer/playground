CREATE TABLE location (
  id NUMBER CONSTRAINT location_pk_id PRIMARY KEY,
  name VARCHAR2(11),
  pid NUMBER CONSTRAINT location_fk_pid REFERENCES location (id) ON DELETE CASCADE
);
CREATE SEQUENCE location_seq
  MINVALUE -1;

INSERT INTO location VALUES(location_seq.nextval, 'root', NULL);
INSERT INTO location VALUES(location_seq.nextval, 'plant1', 0);
INSERT INTO location VALUES(location_seq.nextval, 'plant2', 0);
INSERT INTO location VALUES(location_seq.nextval, 'workshop1', 1);
INSERT INTO location VALUES(location_seq.nextval, 'workshop2', 1);
INSERT INTO location VALUES(location_seq.nextval, 'workshop1', 2);
INSERT INTO location VALUES(location_seq.nextval, 'workshop2', 2);
INSERT INTO location VALUES(location_seq.nextval, 'region1', 3);
INSERT INTO location VALUES(location_seq.nextval, 'region2', 3);
INSERT INTO location VALUES(location_seq.nextval, 'region1', 4);
INSERT INTO location VALUES(location_seq.nextval, 'region2', 4);
INSERT INTO location VALUES(location_seq.nextval, 'line1', 7);
INSERT INTO location VALUES(location_seq.nextval, 'line2', 7);
INSERT INTO location VALUES(location_seq.nextval, 'line1', 8);
INSERT INTO location VALUES(location_seq.nextval, 'line2', 8);
INSERT INTO location VALUES(location_seq.nextval, 'station1', 11);
INSERT INTO location VALUES(location_seq.nextval, 'station2', 11);
INSERT INTO location VALUES(location_seq.nextval, 'station1', 12);
INSERT INTO location VALUES(location_seq.nextval, 'station2', 12);
