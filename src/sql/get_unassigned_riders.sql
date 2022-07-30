SELECT
	rider_id,
	driver_id,
	event_id,
	campus,
	pickup_location
FROM rides
WHERE
	rides.event_id = ?
	AND rides.driver_id IS NULL
	AND rides.campus = ?;
