<!-- omit in toc -->
# Contributing to DEAF

First off, thanks for taking the time to contribute! ❤️

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways to help and details about how this project handles them. Please make sure to read the relevant section before making your contribution. It will make it a lot easier for us maintainers and smooth out the experience for all involved. The community looks forward to your contributions. 🎉

> And if you like the project, but just don't have time to contribute, that's fine. There are other easy ways to support the project and show your appreciation, which we would also be very happy about:
> - Star the project
> - Tweet about it
> - Refer this project in your project's readme
> - Mention the project at local meetups and tell your friends/colleagues

<!-- omit in toc -->
## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [I Have a Question](#i-have-a-question)
- [I Want To Contribute](#i-want-to-contribute)
  - [Submitting A Bug Report](#submitting-a-bug-report)
  - [Submitting A Feature Request](#submitting-a-feature-request)
  - [Submitting Improvements](#submitting-improvements)
- [Join The Project Team](#join-the-project-team)

## Code of Conduct

This project and everyone participating in it is governed by the
[DEAF Code of Conduct](CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code. Please report unacceptable behavior
to <mjhouse@protonmail.com>.

## I Have a Question

> If you want to ask a question, we assume that you have read the available [documentation](https://mjhouse.github.io/deaf/deaf/index.html).

Before you ask a question, it is best to search for existing [issues](https://github.com/mjhouse/deaf/issues) that might help you. In case you have found a suitable issue and still need clarification, you can write your question in this issue. It is also advisable to search the internet for answers first.

If you then still feel the need to ask a question and need clarification, we recommend the following:

- Open an [issue](https://github.com/mjhouse/deaf/issues/new).
- Tag your issue with the `question` label.
- Provide as much context as you can.
- Provide project and system version (if relevant).

We will then take care of the issue as soon as possible.

## I Want To Contribute

> ### Legal Notice <!-- omit in toc -->
> When contributing to this project, you must agree that you have authored 100% of the content, that you have the necessary rights to the content and that the content you contribute may be provided under the project license.

### Submitting A Bug Report

Check to see if the report is actually necessary:

- Make sure that you are using the latest version of the library (the bug may have already been resolved).
- Read the [documentation](https://mjhouse.github.io/deaf/deaf/index.html) carefully to make sure it isn't an error on your side.
- Check to see if there is an existing bug report for your error in the [bug tracker](https://github.com/mjhouse/deaf/labels/bug).

Once you've determined that a bug report is necessary:

- Write a brief description of the bug
- Write down detailed steps to reproduce the bug
- Determine what behavior *should* have happened
- Collect detailed information about the environment (as necessary):
    - Collect error output (if available)
    - Record the OS, platform and version of the host (Windows 10, Ubuntu 18.04 etc.)
    - Get the architecture of the ELF file being inspected (x86, ARM etc.)
    - Check your current Rust version
- Write down anything else that seems relevant but doesn't fit anywhere else

Once you have a clear description of the problem, add a bug report in the [issue tracker](https://github.com/mjhouse/deaf/issues):

- Open an [issue](https://github.com/mjhouse/deaf/issues/new) using the "Bug report" template.
- Fill out the report with the information you collected above.

### Submitting A Feature Request

Check to see if the feature is actually necessary:

- Make sure that you are using the latest version of the library (the feature may have been added).
- Read the [documentation](https://mjhouse.github.io/deaf/deaf/index.html) carefully and find out if the functionality is already available.
- Perform a [search](https://github.com/mjhouse/deaf/issues) to see if the enhancement has already been suggested.
- Find out whether your idea fits with the scope and aims of the project (read the README).

Once you've determined that your feature is not already available and fits the project goals:

- Write a description of the problem that this feature would solve
- Describe the feature as clearly as possible and explain how it would solve the problem
- Gather a list of workarounds or alternatives you've found (if any)
- Build a list of additional notes or useful context for the feature

Once you have a clear description of the feature, add a feature request issue in the [issue tracker](https://github.com/mjhouse/deaf/issues):

- Open an [issue](https://github.com/mjhouse/deaf/issues/new) using the "Feature request" template.
- Fill out the feature with the information you collected above.

### Submitting Improvements

> **Discuss your changes via an issue FIRST**. You may need to create a bug report or a feature request, or there might already be one that you can comment on. Regardless, it's very important that you get some amount of buy-in from other contributors before you start working- it would be terrible if you went to a lot of trouble implementing some cool feature only to find that it doesn't fit with the goals of the project and won't be accepted.

If a new issue is necessary, submit it using the following procedure:

- Open a new [issue](https://github.com/mjhouse/deaf/issues/new).
- Describe the changes you want to see (doc comments, bug fix, or feature).
- Assign the issue to yourself (if that doesn't work, ask to have it assigned to you)
- Tag your issue with the appropriate label (`documentation`,`bug`, or `feature request`).

Once you've discussed the changes and gotten buy-in from other contributors, you can begin to implement your changes. All changes to the code base will use this general workflow

1. Fork the DEAF repository (if you haven't already).
2. Clone your fork locally.
3. Switch to the `development` branch.
4. Implement your changes (see [code](#improving-the-code) and [doc](#improving-the-documentation) sections).
5. Submit a PR against the `development` branch of DEAF

After any issues have been resolved and your PR is accepted:

* Your changes will be merged into `development`.
* At some point (once a day, or every few days) `development` will be merged to `testing`.
* All tests in the codebase will be run after the changes are pushed to `testing`.
* On success, your changes will be merged with `master`.
* Documentation will be rebuilt and deployed.

Congratulations! You helped make DEAF a bit more awesome!

#### Improving The Code

Follow the above general workflow in order to submit changes to DEAF. When you get to step #4 (implementing your changes), do this:

1. Make your code changes (to implement the feature or fix the bug)
2. Validate your changes by writing and running tests
3. Add documentation comments to any new structs/methods
4. Resolve any warnings emitted in the console

Then continue on with step #5.

#### Improving The Documentation

Follow the above general workflow in order to submit changes to DEAF. When you get to step #4 (implementing your changes), do this:

1. Add or change whatever documentation necessary to resolve your issue
2. Verify that any new doc-tests work correctly (just run `cargo test`)
3. Resolve any warnings emitted in the console

Then continue on with step #5.

## Join The Project Team

Send me (Michael House) an email at mjhouse@protonmail.com with the subject "DEAF - Join The Project Team". You may be asked a few questions and if you haven't contributed anything to the code base, you probably won't be accepted. There's not currently any serious evaluation or onboarding process for new team members because there's only one or two real contributors at the moment, although this may change in the future.
