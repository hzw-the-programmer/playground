-- source /home/zhiwenhe/work/python-play-ground/mysql/on_delete_cascade.sql

DROP TABLE IF EXISTS rooms;
DROP TABLE IF EXISTS buildings;

CREATE TABLE buildings (
    building_no INT PRIMARY KEY AUTO_INCREMENT,
    building_name VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL
);

CREATE TABLE rooms (
    room_no INT PRIMARY KEY AUTO_INCREMENT,
    room_name VARCHAR(255) NOT NULL,
    building_no INT NOT NULL,
    FOREIGN KEY (building_no)
        REFERENCES buildings (building_no)
        ON DELETE CASCADE
);

INSERT INTO buildings (building_name, address)
VALUES ('ACME Headquaters', '3950 North 1st Street CA 95134'),
       ('ACME Sales', '5000 North 1st Street CA 95134');

INSERT INTO rooms (room_name, building_no)
VALUES ('Amazon', 1),
       ('War Room', 1),
       ('Office of CEO', 1),
       ('Marketing', 2),
       ('Showroom', 2);

-- DELETE FROM buildings WHERE building_no = 2;
