# Tets 3: Getting the key we just posted to check it was persisted
GET http://localhost:3000/data?advent

HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.value" == "of Spin"
