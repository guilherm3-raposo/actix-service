UPDATE
    `user` AS u
    INNER JOIN `person` AS p ON u.id = p.user_id
SET
    u.`username` = ?,
    u.`email` = ?,
    u.`uuid` = ?,
    p.`firstname` = ?,
    p.`lastname` = ?,
    p.`avatar` = ?
WHERE
    `id` = ?