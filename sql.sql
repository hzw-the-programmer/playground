CREATE TABLE kedas (
  id NUMBER CONSTRAINT kedas_pk_id PRIMARY KEY,
  sn VARCHAR2(14),
  ip
)
CREATE TABLE kedas_slot_port (
  id NUMBER CONSTRAINT kedas_slot_port_pk_id PRIMARY KEY,
  kedas_id NUMBER CONSTRAINT kedas_slot_port_fk_kedas_id REFERENCES kedas (id) ON DELETE CASCADE,
  slot NUMBER,
  port NUMBER,
  channel VARCHAR2(20)
)
