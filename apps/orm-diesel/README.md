# diesel

文档：https://diesel.rs/guides/getting-started

## 安装 Diesel CLI

```bash
$ cargo install diesel_cli
$ cargo install diesel_cli --no-default-features --features postgres
```

## [命令行](https://diesel.rs/guides/getting-started)

### 1.创建数据库配置文件（使用一个名为 `.env` 的工具来为我们管理环境变量）：

```bash
$ echo DATABASE_URL=postgres://postgres:postgres@172.16.61.135:5432/db_rust > .env
```

### 2.创建数据库（如果不存在）以及 Diesel 相关的文件和目录，包括 `migrations` 目录和 `diesel.toml` 配置文件 :

```bash
$ diesel setup 
```

### 3.生成新的模型迁移文件，需要自己在生成的模型文件夹下的sql文件中，编写sql语句 :

```bash
# diesel migration generate [应用名称]
$ diesel migration generate create_posts 
```

SQL示例：

```
/** up.sql **/
-- Your SQL goes here
CREATE TABLE posts
(
    id        SERIAL PRIMARY KEY,
    title     VARCHAR NOT NULL,
    body      TEXT    NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
);

/** down.sql **/
-- This file should undo anything in `up.sql`
DROP TABLE posts
```

### 4.运行数据库迁移，以创建数据库表, 并生成 `src/schema.rs` 文件和 `diesel::table!` 宏代码:

```bash
$ diesel migration run 
```

`src/schema.rs` 示例：

```rust
// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
```

### 5.回滚

```bash
$ diesel migration redo 
```

### 6.生成 `schema.rs` 文件 :

```bash
$ diesel print-schema > src/schema.rs 
```