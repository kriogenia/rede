# Errors

#### failed connection

`rede` was unable to establish a connection with the server. It could be down
or the URL could be wrong.

#### failed request building

A part of your request is not correct, try looking into the cause to discern
the reason.

#### invalid file

One of the files referenced in the request does not exist or can't be read.
The most probable reson is that you defined a path relative to the request file,
but the way rede works the path must be relative to the path from where you are
executing rede.

#### invalid [REQUEST]

The request file specified in the command does not exist or can't be read.

#### invalid url

The parsed URL is invalid

#### unknown request error

Some error has occurred with your request that we didn't expect. You can
[create an issue](https://github.com/kriogenia/rede/issues) in the
repository to help you point out the issue or help us know about it to
discern if we should give it its own error code and help.

#### redirect

The request has reached the maximum number of allowed redirections or some
redirection loop has been detected.

#### spec violation

Two possible options. You have an invalid [TOML](https://toml.io/en/v1.0.0)
or you are breaking the [DSL](request_dsl.md). Follow the error cues to
discern what it could be.

#### wrong http version

The endpoint of your request does not support the HTTP version defined in
your request file.