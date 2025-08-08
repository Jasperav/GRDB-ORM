fn string_var(v: &str) -> String {
    std::env::var(v).unwrap()
}

macro_rules! num_var {
    ($v: ident, $num: ty) => {
        #[allow(dead_code)]
        fn $v(v: &str) -> $num {
            string_var(v).parse().unwrap()
        }
    };
}

num_var!(i32_var, i32);
num_var!(i64_var, i64);
num_var!(i128_var, i128);
num_var!(u8_var, u8);
num_var!(u32_var, u32);
num_var!(u128_var, u128);
num_var!(f32_var, f32);
num_var!(f64_var, f64);
num_var!(usize_var, usize);
num_var!(bool_var, bool);

lazy_static::lazy_static! {
    // This is an absolute path to the location of a SQLite database where the metadata will be extracted from
    pub static ref SQLITE_LOCATION: String = string_var("SQLITE_LOCATION");
    // Either Public or Internal
    pub static ref VISIBILITY: String = string_var("VISIBILITY");
    // This is an absolute path to the output folder of the generated files
    // Note that this crate will always create a subfolder 'generated' of the specified folder
    pub static ref OUTPUT_DIR: String = string_var("OUTPUT_DIR");
    // Output dir to generated android Room entities. Leave NONE if you don't use Android.
    pub static ref OUTPUT_DIR_ANDROID: String = string_var("OUTPUT_DIR_ANDROID");
    // When true, the generated swift files will be formatted by https://github.com/nicklockwood/SwiftFormat
    pub static ref USE_SWIFTFORMAT: bool = bool_var("USE_SWIFTFORMAT");
    // When true, the generated swift files will be autocorrected by https://github.com/realm/SwiftLint
    pub static ref USE_SWIFTLINT: bool = bool_var("USE_SWIFTLINT");
    // When true, all non-primary properties are declared as 'let' (everything is let by default)
    pub static ref ALL_IMMUTABLE: bool = bool_var("ALL_IMMUTABLE");
    // Suffix to append to the generated structs (can be empty)
    pub static ref SUFFIX_SWIFT_STRUCTS: String = string_var("SUFFIX_SWIFT_STRUCTS");
    // Prefix to append to the generated structs (can be empty)
    pub static ref PREFIX_SWIFT_STRUCTS: String = string_var("PREFIX_SWIFT_STRUCTS");
    // The packages to import for each generated file
    // To specify multiple packages, separate them by adding '|' between them, like so: MyPackage|MyPackage2
    pub static ref PACKAGES: String = string_var("PACKAGES");
    // Will find unused indexes + missing indexes for dynamic queries
    pub static ref INDEX_OPTIMIZER: bool = bool_var("INDEX_OPTIMIZER");
    // Add this if you want the SQL queries to be logged in debug mode
    pub static ref ANDROID_PACKAGE_NAME: String = string_var("ANDROID_PACKAGE_NAME");
    // Set to true to log the arguments but this makes the app very slow (this is only executed in debug though)
    pub static ref ANDROID_ENABLE_VERBOSE_SQL_ARGUMENT_LOGGING: bool = bool_var("ANDROID_ENABLE_VERBOSE_SQL_ARGUMENT_LOGGING");
}
