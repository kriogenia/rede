# How to run

`rede` works using a DSL built on top of the [TOML](https://toml.io/en/) specification.
All the documentation regarding rede's DSL can be found on the respective [page](../reference_guide/request_dsl.md).
An easy way to boostrap a request following the DSL is running the
`rede example` command.

It will generate a `example.toml` file looking like this.

```toml
[http]
method = "GET"
url = "https://dogapi.dog/api/v2/facts"

[headers]
Accept = "application/json"
User-Agent = "rede/v0.1.0"

[query_params]
limit = "{{limit}}"

[input_params]
limit.hint = "Number of facts to retrieve"

[variables]
limit = 3
```

Once you have run this command, or redacted your own request, you will be ready
to execute rede.

```shell
rede run <your_file>
```

And that's all you need to start making requests with `rede`, but that's not all
you can do.

