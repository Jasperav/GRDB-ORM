# 🚀 Blazing fast ORM for GRDB.swift

Compatibility: [GRDB 5.8.0](https://github.com/groue/GRDB.swift), Swift 5.4\
License: [MIT](/LICENSE)\
CI: [![asdasd](https://img.shields.io/github/workflow/status/jasperav/GRDB-ORM/GeneratedCode/master?label=Generated%20Swift%20code)](https://github.com/Jasperav/GRDB-ORM/actions?query=workflow%3AGeneratedCode)
[![asdasd](https://img.shields.io/github/workflow/status/jasperav/GRDB-ORM/Parser/master?label=Parser)](https://github.com/Jasperav/GRDB-ORM/actions?query=workflow%3AParser)

*The code generates `struct`s for the mapped tables and all `struct`s implements `Codable` for coding values to/from the database. 
This can NOT (yet) be customized.*

## What can this code do
Examples of what can be generated can be found [here](Parser/generated). 
This contains the most up-to-date generated code based on [this configuration](Parser/src/generate_generated_code.rs).

- Map SQLite tables and columns to Swift `struct`s and properties
- Boilerplate query generation for common actions (CRUD)
- Helper function to generate custom queries, which makes extracting columns from a `select` clause fairly simple.
The queries are validated when generating a Swift `func` for them
- Primary key objects are generated for each `struct` which can in turn be used to query a unique row or to delete a unique row
- [Configurable](#add-your-configuration)
- [Much better performance](#performance) than `Codable` `struct`s in GRDB (without custom initializers)
- Index checker based on dynamic queries (because auto generated queries are exclusively on the primary key index):
  - Automatically finds unused indexes
  - Finds missing indexes
  
## Why this is faster than GRDB
- Generated code knows the properties types at compile time, rather than runtime. It removes the overhead
  GRDB has for checking each property type's type.
- Generated code has literal string queries, which removes the need of building queries at runtime
- Generated code already extracted all metadata of all tables at compile time, no need to do it at runtime
- Generated code decodes rows by using indexes exclusively (so not by name)

## How to use it
1. Make sure you have a SQLite database somewhere that mimics your iOS/macOS database 
   (or point the configuration to your iOS/macOS database directly)
2. Clone this project
3. Navigate inside the Parser dir
4. [Add your configuration](#add-your-configuration)
5. Run the parser: Run `cargo run` in the Parser dir
6. Copy paste the generated files from your output dir in your Xcode project

## Add your configuration
1. Go to the [.env](Parser/config/.env) file and fill at least `SQLITE_LOCATION` and `OUTPUT_DIR`
2. Add your custom type mapping to [custom_mapping](Parser/config/custom_mapping.toml). This is useful if you use 
   Json types* or `UUID`s
3. Add custom queries by adding them to [dyn_queries](Parser/config/dyn_queries.toml). Maybe more queries will
be added over time that are commonly used to the standard generated output.
   
* Values with database type 'TEXT' should conform to protocol Codable. If your type can for some reason not conform to this protocol,
change the database type to 'BLOB' and make sure your type has the following methods:
  
- `func serializedData() throws -> Data {...}`
- `func serializedData(data: Data) throws -> Self {...}`

## Performance
The performance tests can be found in [/GRDBPerformance/GRDBPerformanceTests](/GRDBPerformance/GRDBPerformanceTests) and are ran with the following configuration:
- iPhone 8 simulator
- Release mode
- macOS Big Sur - https://support.apple.com/kb/SP756

The CI also runs the performance tests, to check the current results:
1. Click on 'Actions' tab in this repository in Github
2. Click on workflow 'GeneratedCode'
3. Click on the latest workflow
4. Click on 'build' under 'jobs'
5. Click on 'build and test' step

_These results are not comparable to [GRDB performance tests](https://github.com/groue/GRDB.swift/wiki/Performance) since different `struct`s are used_

- The CRUD operations are ran on 10.000 rows (10.000 inserts/selections/updates/deletes)
- Database resets are also included in the time (because the `measure` block doesn't have some sort of setup method)

<!-- https://www.tablesgenerator.com/markdown_tables -->
|        	| GRDB  	| GRDB-ORM 	|
|--------	|-------	|------------------	|
| Insert 	| 0.749 	| 0.418            	|
| Select 	| 2.175 	| 0.692            	|
| Update 	| 1.106 	| 0.599            	|
| Delete 	| 0.917 	| 0.369            	|

## Tips
- Place the generated files in a separate Swift Package and add a dependency on that package in your main project.
  This will speed up the incremental compilation time.

## Roadmap
- CI that
    - auto update performance table. Can be done by reading out the output from the Xcode test
    - creates TOC
    - creates the executable
    - Put the up to date generated code in the right folder and also in the performance test dir
    - Uses SSH (currently using HTTPS with some hacks to make it work)
- Better error handling of code generation if the client provides bad input
- Automatic migrations. Idea: make a folder 'migrations' and put the SQL inside it. Each file represents a migration.
- Easier custom query writers. Currently, the user has to make sure a lot of conditions are uphold:
    - using the correct return types
    - correct parameterized placeholder values 

Maybe this can be better, but is is really hard to parse SQL, because it can do so much.

## Help contributing
I am active on Github, PR's are always welcome! 

If you want to contribute, it can be useful to quickly see
how various configurations generate different output. If you want to do so, install `cargo` and a Rust toolchain
and checkout [generate_generated_code.rs](/Parser/src/generate_generated_code.rs). You can easily change configuration there without the need of making
changes to an actual database.

Run executable 'new_version' to generate a new version

## Why is this written in Rust and not in Swift?
- I wrote the `SQLiteParser` dependency also in Rust
- Rust has better support for parsing the configuration files (.toml)