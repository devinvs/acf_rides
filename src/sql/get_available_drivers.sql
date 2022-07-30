SELECT
    drivers.event_id,
    drivers.driver_id,
    drivers.seats,
    drivers.vehicle_id,
    drivers.campus,
	SUM(CASE WHEN rides.rider_id IS NULL THEN 0 ELSE 1 END) AS rider_count
FROM drivers
	LEFT JOIN rides ON rides.driver_id = drivers.driver_id
WHERE drivers.event_id = ?
	AND drivers.campus = ?
GROUP BY drivers.driver_id;
