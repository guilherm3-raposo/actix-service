SELECT
    *
FROM
    `user` AS u
    LEFT JOIN `person` AS p ON
        p.user_id = u.id
WHERE
    `id` = ?