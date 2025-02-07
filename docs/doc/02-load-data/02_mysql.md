---
title: Load Data from MySQL
sidebar_label: Load Data from MySQL
description:
  Load Data from MySQL
---

Using `mysqldump` to load data from MySQL into Databend.

### Before you begin

* **MySQL client and mysqldump**
* **Databend:** You will connect to the database and table using MySQL client, please see [How to deploy Databend](/doc/category/deploy).

### Part 1: Dump MySQL table schema and data to file

```sql title="Dump book_db.books table schema and datas"
mysqldump --single-transaction --compact -uroot -proot book_db books > dumpbooks.sql
```

:::tip

mysqldump Options: [--single-transaction](https://dev.mysql.com/doc/refman/8.0/en/mysqldump.html#option_mysqldump_single-transaction) [--compact](https://dev.mysql.com/doc/refman/8.0/en/mysqldump.html#option_mysqldump_compact)

:::

The `dumpbooks.sql` looks like:
```text title='dumpbooks.sql'
CREATE TABLE `books` (
  title varchar(255),
  author varchar(255),
  date varchar(255) 
);
INSERT INTO `books` VALUES ('Transaction Processing','Jim Gray','1992'),('Readings in Database Systems','Michael Stonebraker','2004');
... [snip] ...
INSERT INTO `books` VALUES ('Transaction Processing','Jim Gray','1992'),('Readings in Database Systems','Michael Stonebraker','2004');
```

### Part 2: Load data into Databend from the sql file

```sql
mysql -uroot -h127.0.0.1 -proot -P3307 < dumpbook.sql
```
All tables and data from users will now be loaded into Databend.
