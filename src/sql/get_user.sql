SELECT
    id,
    email,
    fullname,
    password,
    number
FROM users
WHERE id = ?
LIMIT 1;
