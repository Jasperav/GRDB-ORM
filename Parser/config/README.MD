1. Open .env
2. Change OUTPUT_DIR and SQLITE_LOCATION
These two properties should point to the DB in the parent of this dir and the Entities package
3. Clone GRDB-ORM: https://github.com/Jasperav/GRDB-ORM
4. cp the content of this dir inside the GRDB-ORM /Parser/config dir
5. Run the executable in /Parser/compiled