use env_parser::to_lazy_static::LazyTransformDefault;
use env_parser::EnvReader;
use std::env::current_dir;
use std::fs::File;

fn main() {
    let mut transformer = LazyTransformDefault {
        file: File::create(current_dir().unwrap().join("src").join("properties.rs")).unwrap(),
    };
    env_parser::to_lazy_static::read_env(&mut EnvReader {
        env: include_bytes!("./config/.env").to_vec(),
        transformer: &mut transformer,
    });
}
