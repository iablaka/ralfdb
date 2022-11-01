# ralfdb
rafldb is a simple RDBMS (Relational DataBase Management System) based on text files. It is written in Rust. The project is to create a simple sql engine for a graphical software that would be similar to MS-Access.

As of now, I am a newbie in Rust and I never wrote nor worked on any database engines. This is a very naive approach to learn Rust with hope that to optimize things later on. Contributors that can help me progress by commenting, fixing, improving, or optimizing my code are warmly welcome. Same for people with database creation experience or any sort of curious developers willing to participate to an open-source project.


# How to use it?
Before you start, you must set the RALF_PATH environment variable to "..".

Run the rdc executable (rdc stands for **r**alf **d**b **c**ommand line) by typing
```
cd rdc
cargo run
```
Then at prompt
```
rdc> use crm;
```
You should see a message saying you now use the crm database. Then enter
```
rdc> select * from customers;
```
You should see the content of the customers table printed and the time it took to query this database

# Todo
* what about a string with a , in it. should be solved by protecting blocks per quotes
* refacto: col size function should be a method of metadata. Enum SqlCommands
* clean up main.rs adding some modules (formatters, parsers)
* is_valid_query to make sure command is correct SQL => regex
* WHERE clause
* multi-tables queries with manual joins
* JOIN clause
* DELETE, UPDATE, and INSERT
* DROP
* CREATE
* ALTER
