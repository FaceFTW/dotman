# Dotman

`dotman` is a diverging fork of [`SuperCuber/dotter`](https://github.com/SuperCuber/dotter), which now seems to be unmaintained. This fork has some key differences from `dotter` to make it much "lighter" but still retain compatibility with the original tool.

___This divergence is a work in progress, with breaking changes detailed as they are made.___

# Differences from Dotter

- Removal of the "watch" feature
  - This increased size/startup time due `watchexec`'s reliance on Tokio. Also had [bugs](https://github.com/SuperCuber/dotter/issues/196)
- Math Expressions are no longer supported in Handlebars substitutions
  - Doesn't really make sense, also newer versions of `evalexpr` are AGPL and would require license changing
- Handlebars Scripting no longer has a feature flag, is always integrated
  - This feels like a "core" feature that shouldn't really be disabled, and adding a flag makes maintenance a bit annoying



# Installation

No official packages are yet available, but building from source is possible through Cargo:

```sh
cargo build --release
```

The compiled binary (`dotman`) will be in the `target/release` folder.

# Wiki
TODO

Most of the templating and configuration remains the same as `dotter`, use that Wiki for now until I migrate it over:
https://github.com/SuperCuber/dotter/wiki

# Usage

Check out `dotman -h` for the command-line flags that Dotman supports:

```

```

# Contributing
Contributions to Dotman are welcome, whether in the form of a pull request or an issue (for bug repots, feature requests, or other helpful comments)

# Legal Stuff

`dotman` is licensed under the MIT License.
The original source code of dotter was licensed under the Unlicense.