SELECT
    event_id,
    driver_id,
    seats,
    vehicle_id,
    campus
FROM drivers
WHERE event_id = ?
AND campus = ?;
