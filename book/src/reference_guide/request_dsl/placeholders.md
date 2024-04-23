# Placeholders

Placeholders are strings wrapped in double braces, like `{{this}}`. They can be the whole value
of a string or can be a substring, for example: `{{host}}/api/{{version}}/hello`.

## Resolvers

Placeholders can be given a value through different resolvers. These resolvers have a defined
order and the first providing a value will be the one that is used. If no resolver has a value
for the placeholder then the user will be prompted to resolve it.

The order of resolution is:
1. [Input parameters](#input-parameters), these will be input by the user when the request is executed. If the
user doesn't provide a value and the input parameter has a default, that default will be used.
2. [Variables](#variables), these are defined in a [standard table](../request_dsl.md#variables)
similar to query params or headers, but this one is only aimed to provide values for placeholders.