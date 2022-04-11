SELECT
    u.id,
    u.email,
    u.fullname,
    u.password,
    u.number,
    u.campus
FROM rides r
    LEFT JOIN users u ON r.rider_id=u.id
WHERE r.event_id = '?'
    AND r.driver_id = '?'
