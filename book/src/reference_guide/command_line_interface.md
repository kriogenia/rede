# Command Line Interface

The general way of running `rede` is using `rede <subcommand>`. The current supported
subcommands are:

- [run](./command_line_interface/run.md)
- [example](./command_line_interface/example.md)
- `help`, prints the help, same as `rede --help`.

`rede` supports infering for both, subcommands and arguments. That means that
you can use substrings to match a command as long as there's no conflict. For
example, this is valid way of using `rede run` with `--verbose`:

```shell
rede r --verb my_request
```

## Global options

The following options of `rede` can be used with all subcommands.

- `--help`, will print the help of the respective command.
- `--quiet`, evades all standard output, only errors will be printed (to stderr).
- `--verbose`, ups the number of printing messages.
- `--dry-run`, _see for each command_.
- `--no-color`, disables coloring in the outputs.

