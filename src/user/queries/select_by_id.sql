SELECT
    u.*,
    p.*,
    COALESCE(GROUP_CONCAT(r.`value`), '') AS `roles`
FROM
    `user` AS u
    LEFT JOIN `person` AS p ON p.user_id = u.id
    LEFT JOIN `role` AS r ON r.user_id = u.id
WHERE
    `id` = ?
GROUP BY
    `id`