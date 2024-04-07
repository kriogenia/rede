use assert_cmd::Command;
use predicates::prelude::predicate::str::contains;

macro_rules! test_request {
    ($name:ident -> $assert:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let file = format!("tests/inputs/{}", stringify!($name));
            Command::cargo_bin("rede")
                .unwrap()
                .arg("run")
                .arg(file)
                .assert()
                .success()
                .stdout($assert);
        }
    };
}

macro_rules! test_error {
    // Runs the test using the file matching the test name
    ($name:ident $(, $arg:literal)* -> $assert:expr) => {
        test_error!($name <$name>, $($arg),* -> $assert);
    };
    // Runs the test using the `get_simple` file, for test not dependent on the request file
    ($name:ident <>, $($arg:literal),* -> $assert:expr) => {
        test_error!($name <get_simple>, $($arg),* -> $assert);
    };
    // Runs the test using the given file
    ($name:ident <$file:ident>, $($arg:literal),* -> $assert:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let file = format!("tests/inputs/{}", stringify!($file));
            Command::cargo_bin("rede")
                .unwrap()
                .arg("run")
                $(.arg($arg))*
                .arg(file)
                .assert()
                .failure()
                .stderr($assert);
        }
    };
}

test_request!(get_simple -> contains(r#"{"hello":"world"}"#));
test_request!(http_version -> contains(r#"{"http_version":"HTTP/1.0"}"#));

test_error!(invalid_url -> contains("invalid url"));
test_error!(unsupported_http_version -> contains("wrong http version"));
test_error!(timeout<>, "--timeout", "0s" -> contains("timeout"));
