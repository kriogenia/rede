use assert_cmd::Command;
use predicates::prelude::predicate::str::contains;

#[ignore]
#[test]
fn replace_envvar() {
    Command::cargo_bin("rede")
        .unwrap()
        .env("endpoint", "api/request")
        .arg("--no-color")
        .arg("run")
        .arg("--pretty-print=false")
        .arg("tests/inputs/replace_variables")
        .assert()
        .success()
        .stdout(contains(r#""http_version":"HTTP/1.1""#));
}

/* TODO find a way to test input params
#[ignore]
#[test]
fn replace_inputparams() {
    Command::cargo_bin("rede")
        .unwrap()
        .arg("--no-color")
        .arg("run")
        .arg("--pretty-print=false")
        .arg("tests/inputs/replace_inputparams")
        .write_stdin("api/request")
        .assert()
        .success()
        .stdout(contains(r#""http_version":"HTTP/1.1""#));
}
*/
