# rs-project-voult

Запуск из карго
```bash
cargo run -- --config config.json
```
Запуск теста из карго с конфигом для модуля infrastructure
```bash
cargo test -p infrastructure -- projects_sql --  --config ../config.json
```

## sea-orm-cli - миграции
sea-orm-cli migrate generate create_table
sea-orm-cli migrate up
sea-orm-cli generate entity  -o entity/src
sea-orm-cli generate entity -u protocol://username:password@localhost/bakery -o entity/src