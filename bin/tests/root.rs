use assert_cmd::Command;
use predicates::str::contains;

macro_rules! test_failure {
    ($(#[$m:meta])*$name:ident: $($arg:literal),* -> $assert:expr) => {
        $(#[$m])*
        #[test]
        fn $name() {
            Command::cargo_bin("rede")
                .unwrap()
                $(.arg($arg))*
                .assert()
                .failure()
                .stderr($assert);
        }
    };
}

test_failure!(help_by_default: -> contains("--help"));
test_failure!(subcommand_required: "--quiet" -> contains("requires a subcommand"));
test_failure!(quiet_and_verbose_mutually_exclusive: "--quiet", "--verbose", "run" ->
   contains("the argument '--quiet' cannot be used with '--verbose'"));
