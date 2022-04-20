UPDATE rides
SET driver_id = NULL
WHERE driver_id = ? AND event_id = ?;
