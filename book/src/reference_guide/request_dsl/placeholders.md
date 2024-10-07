# Placeholders

Placeholders are strings wrapped in double braces, like `{{this}}`.
They can be the whole value of a string or can be a substring,
for example: `{{host}}/api/{{version}}/hello`.

## Resolvers

Placeholders can be given a value through different resolvers.
These resolvers have a defined order and the first one providing
a value will be the one that is used. If no resolver has a value
for the placeholder then the request will fail unless the flag
`--allow-unresolved` is provided.

The order of resolution is:

1. Environment variables. They must match perfectly the placeholder
and are case-sensitive.
2. [Input parameters](./input_parameters.md), these will be input by
the user when the request is executed.
3. [Variables](../request_dsl/#variables), these are defined in a
[standard table](../request_dsl.md#variables) similar to query params
or headers, but this one is only aimed to provide values for placeholders.

