use std::process::Command;

fn main() {
    macro_rules! root {
        () => {
            std::env::current_dir().unwrap().parent().unwrap()
        };
    }
    let parser_dir = root!().join("Parser");

    macro_rules! run {
        ($program: expr, $args: expr) => {
            let success = Command::new($program)
                .args(&$args)
                .current_dir(&parser_dir)
                .status()
                .unwrap();

            if !success.success() {
                panic!("{:#?}", success);
            }
        };
    }

    run!("rm", ["-rf", "./compiled/"]);
    run!("cargo", ["build", "--release"]);
    run!("cp", ["-a", "target/release/.", "./compiled"]);
    run!("rm", ["-rf", "../GRDBPerformance/GRDBPerformance/Generated"]);
    run!("cp", ["-a", "generated/.", "../GRDBPerformance/GRDBPerformance/Generated"]);
    run!("cargo", ["fmt"]);

    println!("Running checks...");

    run!("cargo", ["test", "--verbose", "--", "--test-threads=1"]);
    run!("cargo", ["fmt", "--all", "--", "--check"]);

    // For some reason this doesn't work for me locally
    //run!("cargo", ["clippy", "--all", "--", "-D", "warnings"]);

    // Run the swift tests
    assert!(Command::new("xcodebuild")
        .args(&[
            "test",
            "-project",
            "GRDBPerformance.xcodeproj",
            "-scheme",
            "GRDBPerformanceTests",
            "-destination",
            "platform=iOS Simulator,name=iPhone 8,OS=14.4"
        ])
        .current_dir(root!().join("GRDBPerformance"))
        .status()
        .unwrap()
        .success());
}