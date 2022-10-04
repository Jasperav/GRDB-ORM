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
            let output = Command::new($program)
                .args(&$args)
                .current_dir(&parser_dir)
                .status()
                .unwrap();

            if !output.success() {
                panic!("Ran: {}, {:#?}", stringify!($program), output);
            }
        };
    }

    run!("cargo", ["test", "--", "--test-threads=1"]);
    run!(
        "rm",
        ["-rf", "../GRDBPerformance/GRDBPerformance/Generated"]
    );
    run!(
        "cp",
        [
            "-a",
            "generated/.",
            "../GRDBPerformance/GRDBPerformance/Generated"
        ]
    );
    run!("cargo", ["fmt"]);
    run!("cargo", ["fmt", "--all", "--", "--check"]);

    // For some reason this doesn't work for me locally
    run!("cargo", ["clippy", "--all", "--", "-D", "warnings"]);

    // Run the swift tests
    assert!(Command::new("xcodebuild")
        .args(&[
            "test",
            "-project",
            "GRDBPerformance.xcodeproj",
            "-scheme",
            "GRDBPerformanceTests",
            "-destination",
            "platform=iOS Simulator,name=iPhone 14 Pro,OS=16.0"
        ])
        .current_dir(root!().join("GRDBPerformance"))
        .status()
        .unwrap()
        .success());
}
