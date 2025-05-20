# retrydis

A Rust-based retry storage module designed for persistent and fixed-sized cache storage solutions.

## Features
- Abstract storage layer
- Fixed-size in-memory cache
- Modular architecture

## TODO
[ ] User layout/ user data separation  
[ ] Event loop (parse commands[Regex])
[ ] LRU cache 

## Objective
```
client = RetrydisClient(url)
create_cmd = "create <storage_type> <storage_name> limit <size>"
cline.create(create_cmd)
client.use(storage_name)
cmd = "get/put/remove"
records = client.exec(cmd) 
```

## Usage
```bash
cargo build
```

