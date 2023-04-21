CREATE TABLE `role` (
    user_id INT(11) NOT NULL,
    value ENUM(
        'SYSTEM',
        'ADMIN',
        'MODERATOR',
        'MANAGER',
        'STAKEHOLDER',
        'USER'
    ) NOT NULL,
    CONSTRAINT role_FK FOREIGN KEY (user_id) REFERENCES `user`(id) ON DELETE CASCADE ON UPDATE CASCADE
);