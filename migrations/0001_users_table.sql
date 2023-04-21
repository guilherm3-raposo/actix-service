CREATE TABLE IF NOT EXISTS `user`  (
	id INT(11) auto_increment NULL,
	username VARCHAR(255) NULL,
	email varchar(255) NULL,
	uuid MEDIUMTEXT NULL,
	CONSTRAINT user_PK PRIMARY KEY (id)
);