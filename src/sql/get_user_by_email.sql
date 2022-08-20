SELECT
    id,
    email,
    fullname,
    password,
    number
FROM users
WHERE email = ?
LIMIT 1;
