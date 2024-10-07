# Contributing to rede on GitHub

First of all, thank you for taking time and interest to contribute to rede.
We want you to have a great experience making your first contribution.

This contribution could be anything from a small fix to a typo in our
documentation or a full feature.

If you would like to contribute, but don't know where to start, checkout the
issues that are labeled
[`good first issue`](https://github.com/kriogenia/rede/contribute)
or
[`help wanted`](github.com/kriogenia/rede/issues?q=is%3Aissue+is%3Aopen++label%3A%22help+wanted%22).

## Code of Conduct

We strive to keep rede an open and welcoming environment.
Please read and follow our [Community Code of Conduct](./CODE_OF_CONDUCT.md).

## Report a bug

If you find a bug in the source code, you can help us by
[submitting an issue](https://github.com/kriogenia/rede/issues/new/choose)
or, even better, a [pull request](#send_a_pull_request) with a fix.

Before you submit a new issue please
[search the archive](https://github.com/kriogenia/rede/issues?q=is%3Aissue+)
to see if your issue has already been reported. Avoiding duplicate issues helps
us focus our time on fixing issues and adding new features.

## Request a new feature

If you think of a new feature that would make rede better for everyone, please
[start a discussion](https://github.com/kriogenia/rede/discussions/new) to
propose the idea.

## Pull Requests

### Commits

* In rede we have a [commit message convention](#commit-messages), but it's not
enforced to contributors, we also enjoy seeing the different types of
messages of different contributors. As long as it's explicative it's good.
* All authors of all commits in a Pull Request must abide by the [Code of Conduct](CODE_OF_CONDUCT.md).
* We follow a linear commit history in our git repositories, a pull request cannot
contain merge commits. To apply upstream changes to a branch, please rebase it
to the base branch.

### Branch names

We follow the `<type>/change` format for branch names, for example `feat/new_body`
or `fix/wrong_header`. There is no strict requirement to follow this but this will
help with the automation of the release drafter.

### Commit Messages

First line of each commit message consists of a type and a subject:

```plain
<type>: <subject>
```

You can add more information in third line onward if you deem it useful.

* `<type>: <subject>` must not be longer that 100 characters.
* type is required, must be in lower case and have one of the below values.
  * `feat`: a new feature
  * `fix`: a fix to a bug in an existing feature
    * Other alternatives to this are: `bug`, `bugfix`
  * `enhancement`: code change that neither fixes a bug nor adds a feature
  * `test`: add missing tests or correct existing tests
  * `doc`/`docs`: a documentation only change
  * `chore`: some minor change that doesn't fall in any of the other types
    * You can also use `tyding`, [see](https://www.oreilly.com/library/view/tidy-first/9781098151232/)

## Helpful References

* [How to contribute to open source](https://opensource.guide/how-to-contribute/)
