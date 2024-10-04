use assert_cmd::Command;
use predicates::prelude::predicate::str::contains;

#[ignore]
#[test]
fn envvar_replace() {
    std::env::set_var("endpoint", "api/request");
    Command::cargo_bin("rede")
        .unwrap()
        .arg("--no-color")
        .arg("run")
        .arg("--pretty-print=false")
        .arg("tests/inputs/variables_replace")
        .assert()
        .success()
        .stdout(contains(r#""http_version":"HTTP/1.1""#));
}
