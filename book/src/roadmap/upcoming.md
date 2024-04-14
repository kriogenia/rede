# Upcoming

These are the next planned features for `rede v0.2`:

## Placeholders

The DSL would start to support strings like `{{this}}` embedded in other
strings or as values of different keys. This placeholders would be later
resolved with the new `variables` table or with input of the user.

## Input parameters

The DSL will accept a new table to specify a series of placeholder that
would be resolved asking the user before running the request. A series of
features are planned for this, but some of them could be delayed.
- Support specification of the expected type.
- Support hints to help the user to provide a good value.
- Support multiple choice.
- Support default values to use if user omits the question.
  - The flag `--all-defaults` would allow to evade the request prompts and
    just use the defaults for everything.
- `--dry-run` to produce a request without executing it, just to see it or
  render it as its own file.

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
