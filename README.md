# ralfdb
Simple RDBMS based on text files

# How to use it?
Before you start, you must set the RALF_PATH environment variable to "..".

Run the rdc executable (rdc stands for **r**alf **d**b **c**ommand line) by typing
```
cd rdc
cargo run
```
Then at prompt
```
rdc> use crm
```
You should see a message saying you now use the crm database. Then enter
```
rdc> select * from customers
```
You should see the content of the customers table printed and the time it took to query this database

# Todo
* manage trailing ;
* quotes or not? 
* what about a string with a , in it
* select some fields not them all
* clean up main.rs adding some modules (formatters, parsers)
