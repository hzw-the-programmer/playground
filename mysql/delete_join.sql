DROP TABLE IF EXISTS chan, pow, dev;

CREATE TABLE dev (
    id INT PRIMARY KEY AUTO_INCREMENT,
    sn CHAR(14) NOT NULL
);

CREATE TABLE chan (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    did INT NOT NULL,
    FOREIGN KEY (did)
        REFERENCES dev (id)
        ON DELETE CASCADE
);

CREATE TABLE pow (
    id INT PRIMARY KEY AUTO_INCREMENT,
    power INT NOT NULL,
    did INT NOT NULL,
    FOREIGN KEY (did)
        REFERENCES dev (id)
        ON DELETE CASCADE
);

INSERT INTO dev (sn) VALUES ('00000000000001'), ('00000000000002'), ('00000000000003');
INSERT INTO chan (name, did) VALUES ('c11', 1), ('c12', 1), ('c21', 2);
INSERT INTO pow (power, did) VALUES (11, 1), (23, 2), (24, 3);

#SELECT * FROM dev AS d INNER JOIN chan AS c ON d.id = c.did;
#SELECT * FROM dev AS d JOIN chan AS c ON d.id = c.did;

#SELECT * FROM dev AS d LEFT JOIN chan AS c ON d.id = c.did;

#SELECT * FROM dev AS d RIGHT JOIN chan AS c ON d.id = c.did;

#DELETE FROM dev WHERE sn = '00000000000001';

#SELECT * FROM dev LEFT JOIN chan ON dev.id = chan.did LEFT JOIN pow ON dev.id = pow.did;

#SELECT * FROM dev LEFT JOIN chan ON dev.id = chan.did LEFT JOIN pow ON chan.did = pow.did;

#DELETE chan, pow FROM dev LEFT JOIN chan ON dev.id = chan.did LEFT JOIN pow ON dev.id = pow.did WHERE dev.sn = '00000000000001';
