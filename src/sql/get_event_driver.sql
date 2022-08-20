SELECT
    u.id,
    u.email,
    u.fullname,
    u.password,
    u.number,
    v.id,
    v.user_id,
    v.color,
    v.make,
    v.model
FROM drivers d
	INNER JOIN rides r ON r.driver_id = d.driver_id
    LEFT JOIN vehicles v ON d.vehicle_id = v.id
    LEFT JOIN users u ON u.id = d.driver_id
WHERE d.event_id = ?
    AND r.rider_id = ?;

