SELECT (
    id,
    email,
    fullname,
    password,
    number,
    campus
) FROM users
WHERE id = '?'
LIMIT 1;
