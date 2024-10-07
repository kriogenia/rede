# Input parameters

`rede` requests can have input parameters that can be used to customize
the request execution based on inputs of the user in execution time.
These parameters are defined in the request file and can be used in the URL,
headers, query parameters, and body as [placeholders](placeholders.md).

The input parameters are defined in a table named `input_params` and can
have any key desired by the user but only those present as placeholder in
the request will be used in the request.

> _WARNING_: environment variables take precedence over input params.
> Any environment variable matching the same key that an input param would
> "disallow it".

The input parameters can be defined with the following optional properties:

- `hint`. A hint to the user when prompted to provide a value.
- ~~`secret`~~, _not yet implemented_.
- ~~`choices`~~, _not yet implemented_.

## Example

We have the following request

```toml
url = "{{host}}/api/{{version}}/user/{{id}}"

[input_params]
host = {}
version.hint = "The version of the API"
id = { hint = "The ID of the resource" }

[variables]
version = "v1"
```

When running this request, the user will be prompted to provide the values
for `host`, `version`, and `id`. For those inputs with a hint, it will be
shown to the user. If the user doesn't provide a value for the values,
the next option for the placeholder will be used ([see](placeholders.md#resolvers)).

Finishing the example, imagine that the user provided the following values:

- `host`: `https://test.api.example.com`
- `version`: _empty_ (so, it will use the default value set in `[variables]` `v1`)
- `id`: `123`

Then this user will be executing the request to `https://test.api.example.com/api/v1/user/123`.
