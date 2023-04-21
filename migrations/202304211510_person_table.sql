CREATE TABLE IF NOT EXISTS `person` (
	`user_id` INT(11) NOT NULL,
	`firstname` varchar(100) NULL,
	`lastname` varchar(100) NULL,
	`avatar` varchar(255) NULL,
	CONSTRAINT person_FK FOREIGN KEY (user_id) REFERENCES `user`(id) ON DELETE CASCADE ON UPDATE CASCADE
);