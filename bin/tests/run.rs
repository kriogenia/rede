use assert_cmd::Command;
use predicates::prelude::*;

macro_rules! test_request {
    ($name:ident, $file:expr, $assert:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let file = format!("tests/inputs/{}", $file);
            Command::cargo_bin("rede")
                .unwrap()
                .arg("run")
                .arg(file)
                .assert()
                .success()
                .stdout(predicate::str::contains($assert));
        }
    };
}

test_request!(get, "get_simple", "world");
