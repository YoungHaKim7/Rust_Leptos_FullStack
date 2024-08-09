# 2) Editor Autocompletion inside #[component] and #[server]
- https://book.leptos.dev/getting_started/leptos_dx.html

Because of the nature of macros (they can expand from anything to anything, but only if the input is exactly correct at that instant) it can be hard for rust-analyzer to do proper autocompletion and other support.

If you run into issues using these macros in your editor, you can explicitly tell rust-analyzer to ignore certain proc macros. For the #[server] macro especially, which annotates function bodies but doesn't actually transform anything inside the body of your function, this can be really helpful.

Starting in Leptos version 0.5.3, rust-analyzer support was added for the #[component] macro, but if you run into issues, you may want to add #[component] to the macro ignore list as well (see below). Note that this means that rust-analyzer doesn't know about your component props, which may generate its own set of errors or warnings in the IDE.

# VSCode settings.json:

```json
"rust-analyzer.procMacro.ignored": {
    "leptos_macro": [
        // optional:
        // "component",
        "server"
    ],
}
```

- VSCode with cargo-leptos settings.json:

```json
"rust-analyzer.procMacro.ignored": {
    "leptos_macro": [
        // optional:
        // "component",
        "server"
    ],
},
// if code that is cfg-gated for the `ssr` feature is shown as inactive,
// you may want to tell rust-analyzer to enable the `ssr` feature by default
//
// you can also use `rust-analyzer.cargo.allFeatures` to enable all features
"rust-analyzer.cargo.features": ["ssr"]
```

# neovim with lspconfig:

```lua
require('lspconfig').rust_analyzer.setup {
  -- Other Configs ...
  settings = {
    ["rust-analyzer"] = {
      -- Other Settings ...
      procMacro = {
        ignored = {
            leptos_macro = {
                -- optional: --
                -- "component",
                "server",
            },
        },
      },
    },
  }
}
```

# Helix, in .helix/languages.toml:

```toml
[[language]]
name = "rust"

[language-server.rust-analyzer]
config = { procMacro = { ignored = { leptos_macro = [
    # Optional:
    # "component",
    "server"
] } } }
```

# Zed, in settings.json:

```json
{
  -- Other Settings ...
  "lsp": {
    "rust-analyzer": {
      "procMacro": {
        "ignored": [
          // optional:
          // "component",
          "server"
        ]
      }
    }
  }
}
```
