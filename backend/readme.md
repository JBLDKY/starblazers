## Server
`cargo run --bin server`

Navigate to: 
`localhost:3030/hello`


## Example of running the Client:
`cargo run --bin client -- --name dylan --count 5`

## Running the dockerized db, frontend and backend
`docker compose --env-file backend/.env up --build -d`
