<div align="center">
<h6>Thank you for your interest in contributing!</h6>
<h1>♾️ Contributing ♾️</h1>

<br />

<!-- prettier-ignore-start -->
[![Version][github-version-img]][github-version-uri]
[![Build Status][github-build-img]][github-build-uri]
[![Downloads][github-downloads-img]][github-downloads-uri]
[![Size][github-size-img]][github-size-uri]
[![Last Commit][github-commit-img]][github-commit-uri]
[![Contributors][contribs-all-img]](#contributors-)
<!-- prettier-ignore-end -->

</div>

<br />

---

<br />

## About

Below are a list of ways that you can help contribute to this project, as well as policies and guides that explain how to get started.

Please review everything on this page before you submit your contribution.

<br />

---

<br />

- [About](#about)
- [Issues, Bugs, Ideas](#issues-bugs-ideas)
- [Contributing](#contributing)
  - [Before Submitting Pull Requests](#before-submitting-pull-requests)
  - [Conventional Commit Specification](#conventional-commit-specification)
    - [Types](#types)
      - [Example 1:](#example-1)
      - [Example 2:](#example-2)
  - [Commiting](#commiting)
  - [Code Style](#code-style)
    - [Rust](#rust)
    - [HTML and JavaScript (`ui/`)](#html-and-javascript-ui)

<br />

---

<br />

## Issues, Bugs, Ideas
Stuff happens, and sometimes as best as we try, there may be issues within this project that we are unaware of. That is the great thing about open-source; anyone can use the program and contribute to making it better.

<br />

If you have found a bug, have an issue, or maybe even a cool idea; you can let us know by [submitting it](https://github.com/pol4rfuchs/ntfy-desktop/issues). However, before you submit your new issue, bug report, or feature request; head over to the [Issues Section](https://github.com/pol4rfuchs/ntfy-desktop/issues) and ensure nobody else has already submitted it.

<br />

Once you are sure that your issue has not already being dealt with; you may submit a new issue at [here](https://github.com/pol4rfuchs/ntfy-desktop/issues/new/choose). You'll be asked to specify exactly what your new submission targets, such as:
- Bug report
- Feature Suggestion

<br />

When writing a new submission; ensure you fill out any of the questions asked of you. If you do not provide enough information, we cannot help. Be as detailed as possible, and provide any logs or screenshots you may have to help us better understand what you mean. Failure to fill out the submission properly may result in it being closed without a response.

<br />

If you are submitting a bug report:

- Explain the issue
- Describe how you expect for a feature to work, and what you're seeing instead of what you expected.
- List possible options for a resolution or insight
- Provide screenshots, logs, or anything else that can visually help track down the issue.

<br />

<div align="center">

[![Submit Issue][btn-github-submit-img]][btn-github-submit-uri]

</div>

<br />

<div align="center">

**[`^        back to top        ^`](#about)**

</div>

<br />

---

<br />

## Contributing
If you are looking to contribute to this project by actually submit your own code; please review this section completely. There is important information and policies provided below that you must follow for your pull request to get accepted.

The source is here for everyone to collectively share and colaborate on. If you think you have a possible solution to a problem; don't be afraid to get your hands dirty.

All contributions are made via pull requests. To create a pull request, you need a GitHub account. If you are unclear on this process, see [GitHub's documentation on forking and pull requests](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request-from-a-fork). Pull requests should be targeted at the master branch.

<br />

### Before Submitting Pull Requests

- Follow the repository's code formatting conventions (see below);
- Include tests that prove that the change works as intended and does not add regressions;
- Document the changes in the code and/or the project's documentation;
- Your PR must pass the CI pipeline;
- When submitting your Pull Request, use one of the following branches:
  - For bug fixes: `main` branch
  - For features & functionality: `development` branch
- Include a proper git commit message following the [Conventional Commit Specification](https://www.conventionalcommits.org/en/v1.0.0/#specification).

<br />

If you have completed the above tasks, the pull request is ready to be reviewed and your pull request's label will be changed to "Ready for Review". At this point, a human will need to step in and manually verify your submission.

Reviewers will approve the pull request once they are satisfied with the patch it will be merged.

<br />

### Conventional Commit Specification

When commiting your changes, we require you to follow the [Conventional Commit Specification](https://www.conventionalcommits.org/en/v1.0.0/#specification). The **Conventional Commits** is a specification for the format and content of a commit message. The concept behind Conventional Commits is to provide a rich commit history that can be read and understood by both humans and automated tools. Conventional Commits have the following format:

<br />

```
<type>[(optional <scope>)]: <description>

[optional <body>]

[optional <footer(s)>]
```

<br />

#### Types
| Type | Description |
| --- | --- |
| `feat` | <br/> Introduce new feature <br/><br/> |
| `fix` | <br/> Bug fix <br/><br/> |
| `deps` | <br/> Add or update existing dependencies <br/><br/> |
| `docs` | <br/> Change website or markdown documents. Does not mean changes to the documentation generator script itself, only the documents created from the generator. <br/><br/><small>E.g: documentation, readme.md or markdown</small> <br /><br /> |
| `build` | <br/> Changes to the build / compilation / packaging process or auxiliary tools such as doc generation<br /><br/><small>E.g: create new build tasks, update release script, etc.</small> <br/><br/> |
| `test` | <br/> Add or refactor tests, no production code change. Changes the suite of automated tests for the app. <br/><br/> |
| `perf` | <br/> Performance improvement of algorithms or execution time of the app. Does not change an existing feature. <br/><br/> |
| `style` | <br/> Update / reformat style of source code. Does not change the way app is implemented. Changes that do not affect the meaning of the code<br /><br/><small>E.g: white-space, formatting, missing semi-colons, change tabs to spaces, etc)</small> <br/><br/> |
| `refactor` | <br/> Change to production code that leads to no behavior difference,<br/><br/><small>E.g: split files, rename variables, rename package, improve code style, etc.</small> <br/><br/> |
| `change` | <br/> Change an existing feature. <br/><br/> |
| `chore` | <br/> Includes technical or preventative maintenance task that is necessary for managing the app or repo, such as updating grunt tasks, but is not tied to any specific feature. Usually done for maintanence purposes.<br/><br/><small>E.g: Edit .gitignore, .prettierrc, .prettierignore, .gitignore, eslint.config.js file</small> <br/><br/> |
| `ci` | <br/> Changes related to Continuous Integration (usually `yml` and other configuration files). <br/><br/> |
| `misc` | <br/> Anything that doesn't fit into another commit type. Usually doesn't change production code; yet is not ci, test or chore. <br/><br/> |
| `revert` | <br/> Revert a previous commit <br/><br/> |
| `remove` | <br/> Remove a feature from app. Features are usually first deprecated for a period of time before being removed. Removing a feature from the app may be considered a breaking change that will require a major version number increment.<br/><br/> |
| `deprecate` | <br/> Deprecate existing functionality, but does not remove it from the app.<br/><br/> |

<br />

##### Example 1:

```
feat(core): bug affecting menu [#22]
^───^────^  ^────────────────^ ^───^
|   |       |                  |
|   |       |                  └───⫸ (ISSUE):   Reference issue ID
│   │       │
│   │       └───⫸ (DESC):   Summary in present tense. Use lower case not title case!
│   │
│   └───────────⫸ (SCOPE):  The package(s) that this change affects
│
└───────────────⫸ (TYPE):   See list above
```

<br />

##### Example 2:
```
<type>(<scope>): <short summary> [issue]
  |       |             |           |
  |       |             |           └─⫸ Reference issue id (optional)
  │       │             │
  │       │             └─⫸ Summary in present tense. Not capitalized. No period at the end.
  │       │
  │       └─⫸ Commit Scope: animations|bazel|benchpress|common|compiler|compiler-cli|core|
  │                          elements|forms|http|language-service|localize|platform-browser|
  │                          platform-browser-dynamic|platform-server|router|service-worker|
  │                          upgrade|zone.js|packaging|changelog|docs-infra|migrations|ngcc|ve|
  │                          devtools....
  │
  └─⫸ Commit Type: build|ci|doc|docs|feat|fix|perf|refactor|test
                    website|chore|style|type|revert|deprecate
```

<br />

### Commiting
If you are pushing a commit which addresses a submitted issue, reference your issue at the end of the commit message. You may also optionally add the major issue to the end of your commit body.

References should be on their own line, following the word `Ref` or `Refs`

```
Title:          fix(core): fix error message displayed to users. [#22]
Description:    The description of your commit

                Ref: #22, #34, #37
```

<br />
<br />

### Code Style

The app is written in Rust (`src-tauri/`); the settings and about windows are plain HTML and JavaScript without a build step (`ui/`).

<br />

#### Rust

Before opening a pull request, run these in `src-tauri/`. The same checks run in the `Build Tauri (Rust)` workflow:

```shell
cargo test
cargo clippy -- -D warnings
```

- Keep logic that does not need a window (parsing, URL building, settings, date formats) in plain functions with unit tests, like `poll.rs`, `store.rs` and `datefmt.rs`.
- Use 4 spaces for indentation and keep lines under about 100 characters.
- Use `snake_case` for functions and variables, `CamelCase` for types.
- Do not log secrets. The API token is only logged as a length.
- New commands that the settings windows call must be added to `build.rs` and to `capabilities/local.json`. The page loaded from the ntfy server may only call `badge_reset`.

<br />

#### HTML and JavaScript (`ui/`)

- Use 2 spaces for indentation and `camelCase` for variables and functions.
- No external libraries or network requests; the pages work offline.

<br />

<br />

<div align="center">

**[`^        back to top        ^`](#about)**

</div>

<br />
<br />

<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->

<!-- BADGE > VERSION > GITHUB -->
  [github-version-img]: https://img.shields.io/github/v/release/pol4rfuchs/ntfy-desktop?logo=GitHub&label=Version&color=ba5225
  [github-version-uri]: https://github.com/pol4rfuchs/ntfy-desktop/releases

<!-- BADGE > GITHUB > BUILD -->
  [github-build-img]: https://img.shields.io/github/actions/workflow/status/pol4rfuchs/ntfy-desktop/build-tauri.yml?branch=main&logo=github&logoColor=FFFFFF&label=Build&color=%23278b30
  [github-build-uri]: https://github.com/pol4rfuchs/ntfy-desktop/actions/workflows/build-tauri.yml

<!-- BADGE > GITHUB > DOWNLOAD COUNT -->
  [github-downloads-img]: https://img.shields.io/github/downloads/pol4rfuchs/ntfy-desktop/total?logo=github&logoColor=FFFFFF&label=Downloads&color=376892
  [github-downloads-uri]: https://github.com/pol4rfuchs/ntfy-desktop/releases

<!-- BADGE > GITHUB > REPO SIZE -->
  [github-size-img]: https://img.shields.io/github/repo-size/pol4rfuchs/ntfy-desktop?logo=github&label=Size&color=59702a
  [github-size-uri]: https://github.com/pol4rfuchs/ntfy-desktop

<!-- BADGE > GITHUB > COMMIT -->
  [github-commit-img]: https://img.shields.io/github/last-commit/pol4rfuchs/ntfy-desktop?logo=conventionalcommits&logoColor=FFFFFF&label=Last%20Commit&color=313131
  [github-commit-uri]: https://github.com/pol4rfuchs/ntfy-desktop/commits/main/

<!-- BADGE > ALL CONTRIBUTORS -->
  [contribs-all-img]: https://img.shields.io/github/all-contributors/pol4rfuchs/ntfy-desktop?logo=contributorcovenant&color=de1f6f&label=contributors
  [contribs-all-uri]: https://github.com/all-contributors/all-contributors


<!-- BADGE > BUTTON > SUBMIT ISSUES -->
  [btn-github-submit-img]: https://img.shields.io/badge/submit%20new%20issue-de1f5c?style=for-the-badge&logo=github&logoColor=FFFFFF
  [btn-github-submit-uri]: https://github.com/pol4rfuchs/ntfy-desktop/issues

<!-- prettier-ignore-end -->
<!-- markdownlint-restore -->
