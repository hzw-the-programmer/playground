-- +migrate Up
CREATE TABLE devices_info (
    id INT AUTO_INCREMENT,
    parent_id INT NOT NULL,
    sn VARCHAR(20) NOT NULL,
    mac VARCHAR(20),
    ip CHAR(15) NOT NULL,
    power INT NOT NULL,
    type INT NOT NULL,
    status CHAR(1) NOT NULL,
    level INT NOT NULL,
    createtime INT NOT NULL,
    updatetime INT NOT NULL,
    devaddr VARCHAR(20),
    port INT,
    version VARCHAR(11),
    PRIMARY KEY (id)
);

CREATE TABLE channels_info (
    id INT AUTO_INCREMENT,
    device_id INT NOT NULL,
    slot INT NOT NULL,
    port INT NOT NULL,
    type INT NOT NULL,
    createtime INT NOT NULL,
    updatetime INT NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE mpoint (
    id INT AUTO_INCREMENT,
    pid INT NOT NULL,
    ciid INT NOT NULL,
    name VARCHAR(20) COLLATE utf8mb4_unicode_ci NOT NULL,
    starttime INT NOT NULL,
    endtime INT NOT NULL,
    x_axis DOUBLE NOT NULL,
    y_axis DOUBLE NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE mpoint_realtime_status (
    id INT AUTO_INCREMENT,
    mpoint_id INT NOT NULL,
    swiftnum INT NOT NULL,
    raw_status INT NOT NULL,
    real_status INT NOT NULL,
    alarm_level INT NOT NULL,
    time INT NOT NULL,
    createtime INT NOT NULL,
    data FLOAT,
    PRIMARY KEY (id)
);

CREATE TABLE mpoint_status (
    id INT AUTO_INCREMENT,
    mpoint_id INT NOT NULL,
    swiftnum INT NOT NULL,
    raw_status INT NOT NULL,
    real_status INT NOT NULL,
    alarm_level INT NOT NULL,
    time INT NOT NULL,
    createtime INT NOT NULL,
    data FLOAT,
    PRIMARY KEY (id)
);

CREATE TABLE mpoint_data (
    id INT AUTO_INCREMENT,
    mpoint_id INT NOT NULL,
    swiftnum INT NOT NULL,
    data FLOAT NOT NULL,
    time INT NOT NULL,
    createtime INT NOT NULL,
    is_pad INT,
    PRIMARY KEY (id)
);

-- +migrate Down
