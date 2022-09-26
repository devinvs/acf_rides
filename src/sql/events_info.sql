SELECT
    e.name,
    SUM(CASE WHEN r.rider_id IS NULL THEN 0 ELSE 1 END) as riders,
    SUM(CASE WHEN r.driver_id IS NULL THEN 1 ELSE 0 END) as unassigned
FROM events e
    LEFT JOIN rides r ON r.event_id = e.id
GROUP BY e.name;
