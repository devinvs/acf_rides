SELECT
    u.id,
    u.email,
    u.fullname,
    u.password,
    u.number,
    r.campus,
    r.pickup_location
FROM rides r
    LEFT JOIN users u ON u.id = r.rider_id
WHERE r.event_id = '?'
    AND r.driver_id = '?';
