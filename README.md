# rs-web
cargo run -- --config config.sample.json
cargo test -p infrastructure -- --  --config config.json

## sea-orm-cli
sea-orm-cli migrate generate create_table
sea-orm-cli migrate up
sea-orm-cli generate entity  -o entity/src
sea-orm-cli generate entity -u protocol://username:password@localhost/bakery -o entity/src