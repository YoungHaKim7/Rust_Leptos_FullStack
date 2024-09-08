# link

- [leptos setting](#leptosrust)

- [VSCode 세팅](#vscode-settingsjson)

- [`trunk serve --open으로 작동시키기`](#cargo-install-cargo-generate)


<hr>

- [암기해야할 주요 기능들](#암기해야할-주요-기능들)
  - ReadSignal
  - WriteSignal

<hr>

# Leptos(Rust)[|🔝|](#link)
https://book.leptos.dev/

- awesome-leptos 
  - https://github.com/leptos-rs/awesome-leptos

# cargo install cargo-generate[|🔝|](#link)

```
trunk serve --port 3000 --open
```

# VSCode `settings.json`:[|🔝|](#link)
```json

"rust-analyzer.procMacro.ignored": {
    "leptos_macro": [
        // optional:
        // "component",
        "server"
    ],
}

```

# 암기해야할 주요 기능들[|🔝|](#link)
- ReadSignal
  - https://docs.rs/leptos/latest/leptos/struct.ReadSignal.html

- WriteSignal
  - https://docs.rs/leptos/latest/leptos/struct.WriteSignal.html
