SELECT
    id,
    email,
    fullname,
    password,
    number,
    campus
FROM users
WHERE email = ?
LIMIT 1;
