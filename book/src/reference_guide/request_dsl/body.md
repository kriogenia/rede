# Body

The `[body]` table accepts a series of keys representing the different
types that could be submitted via HTTP. **Only one** of those types
can be defined, attempting to use more than one would render the
request invalid.

In this first version only four types are supported, but all these
types in conjunction with the appropriate headers are enough to
represent all possible mime types.

## binary

Must contain the path to the file that will be sent as value.
**IMPORTANT**, the path must be relative to the point where `rede`
will be executed, not relative to the request file. Future
implementations could change this behavior, but keep in mind
that this is the current state. A valid alias for the key is `file`.

If no `Content-Type` is set in the request, using this type will
set it to `application/octet-stream`.

```toml
body.binary = "$HOME/Videos/la caida de Edgar.mp4"
```

## raw

Must contain as a value the _string_ representing the body content.
_Tip: TOML supports multiline strings for your JSONs and XMLs_.
A valid alias for the key is `text`.

If no `Content-Type` is set in the request, using this type
will set it to `text/plain`.

```toml
[body]
raw = """
{
  "song": "Bohemian Rhapsody",
  "group": "Queen",
  "awesome": true
}
"""
```

## x_www_form_urlencoded

Similar to [query params](../request_dsl.md#query_params).
**Free** table supporting everything except datetimes and tables
as value. Array will be transformed into a comma-separated list of values.
A valid alias for the key is `form_url_encoded`.

If no `Content-Type` is set in the request, using this type will
set it to `application/x-www-form-urlencoded`.

```toml
[body.x-www-form-urlencoded]
username = "VeryDivorcedMan"
country = "za"
```

## multipart_form_data

**Free** table but all its keys must be of type _table_ having a single key of:

- `text` with the text content.
- `file` with the path to the binary file. Like in [binary](#binary)
the path must be relative to the point of execution, not to the request file.

Having a single form key with two possible types would be invalid. A valid
alias for the key is `form_data`.

If no `Content-Type` is set in the request, using this type will set it
to `multipart/form-data`.
