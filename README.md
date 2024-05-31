# axum-jwt

Compile and run the Axum server:
```
cargo run --bin axum-auth
```

Install Slumber:
https://slumber.lucaspickering.me/

See the slumber.yml file in the main directory...

Run Slumber commands:
`$ slumber` then use the UI to run APIs
OR
```
$ slumber request -p local public_view | jq

$ slumber request -p local get_token | jq
{
  "data": {
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImpvaG5kb2VAZ21haWwuY29tIiwiZXhwIjoxNzE3MTc0MTg5fQ.CmJoHeHBT_fi-VMBgoVjaXEN_dV1CNlxHSbP7X9md2Y"
  },
  "success": true
}

$ slumber request -p local secret_view | jq
{
  "data": {
    "message": "No token provided"
  },
  "success": false
}
```