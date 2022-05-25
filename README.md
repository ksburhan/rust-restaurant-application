# Rust Restaurant Application
## Problem Description

- Simulate a restaurant application running on a server, accepting requests and API calls from clients (tablets carried by restaurant staffs)
- Requests manipulate dishes that have been ordered for certain tables
- Dishes need a random amount between 5-15 minutes to be prepared
- Requests are done with REST API calls
- Possible Requests:
    - Add Items: adds multiple Items to a specific table

    - Delete Items: deletes a specific Item from a specific table

    - Query All Items: returns all current Items, that are being prepared for a specific table

    - Query Item: returns a specific Item, that is being prepared for a specific table

- Multiple Clients can send requests
- Clients simulate API calls in a loop

## Usage 
With a defined `rust-toolchain` file, cargo automatically uses the `nightly` build to run the applications. (Required for rocket)
### Server
```
cd [root]
cargo run -p server
```
### Client
```
cd [root]
cargo run -p client
```
## Used Technology

- rocket: setting up a webserver to route HTTP requests and send back responses
- lazy_static: to setup a global reference for the `OrderService`
- serde_json: to deserialize and serialize
- reqwest + tokio: for instancing a client to send HTTP requests to the server

## Current Issues
- Due to considering the time limit and lack of rust knowledge, Items are not being removed once they finished cooking. E.g. the field `finished_at` of the struct `Item` has a timestamp smaller than the current time.

- For the simulation of the clients a `client` binary has been created, however it is not using multiple threads to simulate the restaurant staff. Instead, multiple instances of the `client` binary needs to be opened.
## API Calls
| Tag | Method | Endpoint | Body Data | 
|-----|--------|----------|------------|
| Add items | POST | /api/v1/table/[table_id]/items  | table_id = i64, items = Vec\<String> |
| Remove item | DELETE | /api/v1/table/[table_id]/items/[item_id] | table_id = i64, item_id = i64
| Query all table items | GET | /api/v1/table/[table_id] | table_id = i64
| Query item | GET | /api/v1/table/[table_id]/items/[item_id] | table_id = i64, item_id = i64

## Output
### Client
The response body of any request (Items on a table)
### Server
All rocket logs for incoming requests