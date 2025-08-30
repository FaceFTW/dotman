# `link-cfg`

`link-cfg` is an opinionated, diverging fork of [`SuperCuber/dotter`](https://github.com/SuperCuber/dotter), which now seems to be unmaintained. This fork has some key differences from `dotter` to make it much "lighter" and work the way _I_ want it to, but still retain compatibility with the original tool.

___This divergence is a work in progress, with breaking changes detailed as they are made.___

# Differences from Dotter

For each change, I provide a bit of my reasoning, so before you open an issue, read here!

- Removal of the "watch" feature
> This increased size/startup time due `watchexec`'s reliance on Tokio. Also had [bugs](https://github.com/SuperCuber/dotter/issues/196)
- Math Expressions are no longer supported in Handlebars substitutions
> Doesn't really make sense, also newer versions of `evalexpr` are AGPL and would require license changing
- Handlebars Scripting no longer has a feature flag, is always integrated
> This feels like a "core" feature that shouldn't really be disabled, and adding a flag makes maintenance a bit annoying
- Logging changes
  - More terminal output is displayed by default (INFO level)
  - Some log calls have changed levels depending on utility
  - Single verbosity level prints TRACE + DEBUG logs
  - TRACE logs are removed overall.
  - Using [`env-logger`](https://github.com/rust-cli/env_logger) as a personal preference
> The above changes are more of a personal preference. I'd like to see what `link-cfg` is doing when I do deploy/rollback. `--quiet` is still available as an option for those who want it.
>
> The general methodology I plan to put when it comes to reorganizing INFO and lower messages are as follows:
> - INFO - information about the general actions `link-cfg` is taking
> - DEBUG - extra information like detected state that is more useful in certain situations
> - TRACE - something to insert when working in the code and modifying something


# Installation

No official packages are yet available, but building from source is possible through Cargo:

```sh
cargo build --release
```

The compiled binary (`link-cfg`) will be in the `target/release` folder.

# Wiki
TODO

Most of the templating and configuration remains the same as `dotter`, use that Wiki for now until I migrate it over:
https://github.com/SuperCuber/dotter/wiki

# Usage

Check out `link-cfg -h` for the command-line flags that `link-cfg` supports:

```

```

# Contributing
Contributions to link-cfg are welcome, whether in the form of a pull request or an issue (for bug repots, feature requests, or other helpful comments)

# Legal Stuff

`link-cfg` is licensed under the MIT License.
The original source code of `SuperCuber/dotter` was licensed under the Unlicense.