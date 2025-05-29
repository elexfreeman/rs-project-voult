# rs-project-voult

Запуск из карго
```bash
cargo run -- --config config.json
```

Запуск теста из карго с конфигом для модуля infrastructure
```bash
cargo test -p infrastructure -- projects_sql --  --config ../config.json
```

Запуск теста из карго с конфигом для модуля telegram_user_data с выводом stdout
```bash
cargo test -p helpers -- telegram_user_data --nocapture --  --config ../config.json
```

Компилиция релиза с помощью docker-compose
```bash
docker-compose up
```

Добавить библитеку
```bash
cargo new --lib some-lib-name
```
## sea-orm-cli - миграции
Создайте .env со следующим содержимым
```bash
DATABASE_URL=postgres://user:pass@host:port/db_name
```
### Установка sea-orm-cli
```bash
cargo install sea-orm-cli@1.1.0
```
### запуск миграции
```bash
sea-orm-cli migrate up
```
### генерация миграции
```bash
sea-orm-cli migrate generate create_table
sea-orm-cli generate entity  -o infrastructure/src/entity
sea-orm-cli generate entity -u protocol://username:password@localhost/bakery -o entity/src
```