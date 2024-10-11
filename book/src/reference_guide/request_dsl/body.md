# Body

The `[body]` table accepts a series of keys representing the different
types that could be submitted via HTTP. **Only one** of those types
can be defined, attempting to use more than one would render the
request invalid.

`rede` supports mainly four types of body listed bellow, but combining
these types with the appropriate `Content-Type` headers every mime
type can be used.

Aside from those four main types, `rede` also supports some convenience
types that works like one of the main types but also setting the correct
`Content-Type`.

## Main types

### binary

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

### raw

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

### x_www_form_urlencoded

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

### multipart_form_data

**Free** table but all its keys must be of type _table_ having a single key of:

- `text` with the text content.
- `file` with the path to the binary file. Like in [binary](#binary)
the path must be relative to the point of execution, not to the request file.

Having a single form key with two possible types would be invalid. A valid
alias for the key is `form_data`.

If no `Content-Type` is set in the request, using this type will set it
to `multipart/form-data`.

## Convenience types

The following convenience types can also be used to set a body. The usage is
exactly the same as for their main types. For example, to create a JSON body
you can just do the following:

```toml
[body]
json = """
{
  "game: "Balatro",
  "genres": [ "card_game", "strategy", "rogue_like", "cocaine" ]
}
"""
```

The currently supported types are the following, if you want a new type to be
added comment or contribute as it's indicated in this
[issue](https://github.com/kriogenia/rede/issues/63).

| Key | Type | MIME |
|-----|------|------|
| `json` | raw | `application/json` |
| `xml` | raw | `application/xml` |
