### Docker database delete and start

```shell
docker compose down && docker compose up -d
```

### Add new migration file

```shell
surrealdb-migrations create InitDemoData
```

or with down script

```shell
surrealdb-migrations create InitDemoData --down
```

### Apply or undo migration

Apply all migrations

```shell
surrealdb-migrations apply
```

Revert all migrations

```shell
surrealdb-migrations apply --down 0
```

Revert single migration

```shell
surrealdb-migrations apply --down 20230915_233844_InitDemoData
```