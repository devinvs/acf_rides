SELECT
    v.id,
    v.user_id,
    v.color,
    v.make,
    v.model
FROM vehicles v
WHERE v.driver_id = '?';
