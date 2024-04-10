use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn help_by_default() {
    Command::cargo_bin("rede")
        .unwrap()
        .assert()
        .failure()
        .stderr(contains("--help"));
}

#[test]
fn subcommand_required() {
    Command::cargo_bin("rede")
        .unwrap()
        .arg("--quiet")
        .assert()
        .failure()
        .stderr(contains("requires a subcommand"));
}

#[test]
fn quiet_and_verbose_exclusive() {
    Command::cargo_bin("rede")
        .unwrap()
        .arg("--quiet")
        .arg("--verbose")
        .arg("run")
        .assert()
        .failure()
        .stderr(contains(
            "the argument '--quiet' cannot be used with '--verbose'",
        ));
}
