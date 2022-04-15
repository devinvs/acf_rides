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
    LEFT JOIN rides r ON r.event_id = e.id
WHERE r.rider_id = ?
ORDER BY e.time;

