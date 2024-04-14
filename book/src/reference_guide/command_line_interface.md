# Command Line Interface

The general way of running `rede` is using `rede <subcommand>`. The current supported
subcommands are:
- [run](./command_line_tool/run.md)
- [example](./command_line_tool/example.md)
- `help`, prints the help, same as `rede --help`.

`rede` supports infering for both, subcommands and arguments. That means that you substrings
can match a command as long as there's no conflict. For example, this is valid way of using
`rede run` with `--verbose`:

```shell
rede r --verb my_request
```

## Global options

The following options of `rede` can be used with all subcommands.
- `--verbose`, ups the number of printing messages.
- `--quiet`, evades all standard output, only errors will be printed in stderr.
- `--no-color`, disables coloring in the outputs.
- `--help`, will print the help of the respective command.