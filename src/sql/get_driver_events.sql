SELECT
    e.id,
    e.name,
    e.time,
    e.address1,
    e.address2,
    e.city,
    e.state,
    e.zipcode,
    e.creator_id
FROM events e
    LEFT JOIN drivers d ON d.event_id = e.id
WHERE d.driver_id = ?
ORDER BY e.time;

