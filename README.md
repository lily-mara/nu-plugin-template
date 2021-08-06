# Nushell plugin Template

This template is intended to be used with [cargo-generate] in order to quickly
bootstrap nushell plugin projects. See the [nushell plugin documentation] for
details.

## Usage:

```
> cargo generate --git https://github.com/lily-mara/nu-plugin-template
> cd {{ project-name }}
> cargo build
> nu plugin --load ./target/debug/
> {{ plugin_name }}
───┬───────────────────────────────┬───┬─────────
 # │               a               │ b │    c
───┼───────────────────────────────┼───┼─────────
 0 │ This is an example output row │ 3 │ 99.9990
───┴───────────────────────────────┴───┴─────────
```

## Config values

- `plugin_name` - all nushell plugins are binaries with the name format
`nu_plugin_SOMETHING`. This is how nushell discovers them. You need to tell this
generator what that `SOMETHING` is. If you enter `random` as the plugin name,
your binary will be called `nu_plugin_random`.

- `plugin_struct` - name of the struct that implements the [`Plugin`] trait from
`nu-plugin` crate.

[cargo-generate]: https://github.com/cargo-generate/cargo-generate
[`Plugin`]: https://docs.rs/nu-plugin/0.35.0/nu_plugin/trait.Plugin.html
[nushell plugin documentation]: https://www.nushell.sh/contributor-book/plugins.html
