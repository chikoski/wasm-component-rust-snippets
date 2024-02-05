GET http://localhost:3000/
HTTP 200
[Asserts]
header "Content-Type" == "text/plain"
body == "Hello, Spin"
