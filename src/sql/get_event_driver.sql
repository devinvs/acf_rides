SELECT
    u.id,
    u.email,
    u.fullname,
    u.password,
    u.number,
    u.campus,
    v.id,
    v.user_id,
    v.color,
    v.make,
    v.model
FROM drivers d
    LEFT JOIN vehicles v ON d.vehicle_id = v.id
    LEFT JOIN users u ON u.id = d.driver_id
WHERE d.event_id = ?
    AND d.driver_id = ?;

