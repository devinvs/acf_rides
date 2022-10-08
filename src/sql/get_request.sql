SELECT
    user_id,
    reset_id,
    request_time
FROM resets
WHERE reset_id = ?
LIMIT 1;
