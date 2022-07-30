UPDATE events
SET
    name=?,
    time=?,
    address1=?,
    address2=?,
    city=?,
    state=?,
    zipcode=?
WHERE id = ?;
