# Rede

`rede` (galician word for "net") attempts to be a command-line utility to
help run suites of requests to ease the testing of REST APIs. The main inspiration
for this project comes from [Bruno](todo) and its command line tool.

The idea behind this project is to have a suite of text files representing HTTP requests
that will be picked and executed by `rede`, allowing for easy sharing and control versioning
of these requests. It should also end up being a valid CI/CD tool to test APIs.

## The plan

Imagine the following file `./request.toml` in your project.

```toml
[metadata]
name = "Add new movie"
api_version = "v1"

[params]
title = "string"

[params.release_date]
type = "string"
hint = 'Release date in format "YYYY-MM-DD"'
default = '{{utils.today("YYYY-MM-DD")}}'

[http]
method = "POST"
url = "{{host}}/v1/movies"

[header]
Content-Type = "application/json"
Authorization = "Bearer {{token}}"

[body]
json = '''
{
  "title": {{title}},
  "release_date": {{release_date}},
  "debug": true
}
'''
```

And imagine that you also have a `./env/local.toml` file like this:

```toml
[env]
host = "http://localhost:8080"
token = "MyPersonalToken"
```

Then from your project you could run `rede` and this would happen:

```shell
$ rede run --env local request
> Please, insert: `title` (string):
$ The Lion King
> Please, insert: Release date in format "YYYY-MM-DD"
> Send empty line to use default: "2024-02-29"
$ 1994-10-07
> Sending request: 
> POST localhost:8080/v1/movies
> - Headers:
> Content-Type application/json
> Authorization Bearer MyPersonalToken
> - Body:
> { "title": "The Lion King", "release_date": "1994-10-07", "debug": true }
>
> Response:
> 201 Created
> - Headers:
> Content-Type application/json
> - Body:
> { "id": 1, "title": "The Lion King", "release_date": "1994-10-07" }
```

This is the idea behind `rede`, creating parametrizable and reusable request in TOML
format that can be used from the command line to test you APIs.

But there's still a long road ahead before this becomes a reality.