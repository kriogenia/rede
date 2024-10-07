# Upcoming

These are the next planned features for `rede v0.3`:

## Improve input parameters

Input parameters right now are kinda limited, future versions will
allow for a bit more customization adding:

- `secret` to accept parameters that should be hidden
- `choices` to allow selection between a series of values.
- `type` to constraint the input value.

## Improved verbosity

We can currently have only three levels of verbosity, which is really limited,
specially considering than one of those is "none at all". That's why next version
the plan is to future five levels of verbosity.

- `quiet`, like the current one, prints nothing at all.
- `discrete`, prints only the body of the request, what the standard currently does.
  - It's possible that this one could be used as default when using `rede run` with stdin
    to allow easier piping.
- `standard`, will print the opening, the status code and the body.
- `verbose`, will be reduced from before. Will print the same as standard plus the
  method and url of the request and the headers of the response.
- `stfu`, will have everything like the current verbose.

## Header generation

Right now `rede` generates automatically some headers before executing the
request. It makes sense to offer a way of turning down this feature.
On top of that, right now rede doesn't generate a `User-Agent` header, and it should.

## New body types

It would be a nice addition to support some new body keys to generate some
common body types like JSON and XML without having to manually set the content
type header.
