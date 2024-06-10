## DB MySQL

Добавить миграцию с откатом: `sqlx migrate add -r <name>`  
Запустить миграции: `sqlx migrate run`  
Запустить конкретную миграцию `sqlx migrate run <name>`  
Откат миграции: `sqlx migrate revert`