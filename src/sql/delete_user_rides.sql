UPDATE rides
SET rider_id = NULL
WHERE rider_id = ? AND event_id = ?;
