# rede run

To execute requests in `rede` you need to use `rede run`.
This command accepts a path to a file containing a valid rede request.
The `.toml` extension is optional, but rede will only attempt to load
files with that extension. By default, it will print the body of
the response, or the status code if there's no body in the response.

```shell
rede run my_request
```

`rede run` is also ready to work with the stdin, so you can also use it in pipes:

```shell
cat my_request | rede run > response.json
```

## Redirections

`rede run` will automatically follow redirect response to arrive at
the pointed resource. That means that unless this behavior is overridden
`run` can't return 3xx responses. To disable this behavior you can use
the `--no-redirect` flag.

On top of that, `run` will throw an error after 10 redirections
followed in the same request, you can override this value with `--max-redirects <value>`

## Verbosity

When using `rede run` with the different verbosity options this is what
it will print (each level will print the same as the previous plus
what is described):

- `quiet`, will only print errors to stderr. The waiting spinner can show
but it will be deleted.
- `standard`, will print the response body. If it's empty, the response status code.
- `verbose`, will print the request that it's being sent (including headers and body)
and the whole response received (status, headers and body).

## Other options

`rede run` supports the following options:

- `--pretty-print`, specifies if formatting and jump lines should be
applied to the response body. It's enabled by default, but it can be disabled
via `--pretty-print=false`
- `--timeout`, sets the maximum duration that the client should wait before giving
a timeout. For example, `--timeout 3s` to wait max 3 seconds.

On top of that, it support the global `--dry-run` flag, this will execute all
the steps to build the request but won't execute it. It's a good way to test
your placeholders.

