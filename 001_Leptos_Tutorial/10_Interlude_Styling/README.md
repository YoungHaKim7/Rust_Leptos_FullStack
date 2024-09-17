- https://book.leptos.dev/interlude_styling.html

<hr>

# Scoped CSS style imports for rust. 
- https://github.com/basro/stylance-rs

- Install stylance cli:

```
cargo install stylance-cli
```

```
cargo add stylance -F nightly
  
```

# stylance file구성및 세팅

- dist & target은 만들어지는 폴더(index.css로 변환되서 들어감)
```
eza --icons -la -TL2
drwxr-xr-x    - gy-gyoung 18 Sep 00:10  .
drwxr-xr-x    - gy-gyoung 17 Sep 22:32 ├──  .cargo
.rw-r--r--   40 gy-gyoung 17 Sep 22:32 │  └──  config.toml
.rw-r--r-- 1.4k gy-gyoung 17 Sep 22:32 ├──  .gitignore
.rw-r--r--  41k gy-gyoung 17 Sep 22:32 ├──  Cargo.lock
.rw-r--r--  208 gy-gyoung 17 Sep 22:32 ├──  Cargo.toml
drwxr-xr-x    - gy-gyoung 18 Sep 00:03 ├──  dist
.rw-r--r--  25k gy-gyoung 18 Sep 00:03 │  ├──  a04_stylance_test-b1ea5863eb3f8f81.js
.rw-r--r-- 1.2M gy-gyoung 18 Sep 00:03 │  ├──  a04_stylance_test-b1ea5863eb3f8f81_bg.wasm
.rw-r--r--  145 gy-gyoung 18 Sep 00:03 │  ├──  index-1282049c66026fcf.css
.rw-r--r-- 5.2k gy-gyoung 18 Sep 00:03 │  └──  index.html
.rw-r--r--  145 gy-gyoung 17 Sep 23:44 ├──  index.css
.rw-r--r--  127 gy-gyoung 17 Sep 23:01 ├──  index.html
.rw-r--r-- 3.3k gy-gyoung 17 Sep 22:32 ├──  justfile
.rw-r--r--  103 gy-gyoung 17 Sep 23:44 ├──  README.md
.rw-r--r--   69 gy-gyoung 17 Sep 22:32 ├──  rust-toolchain.toml
drwxr-xr-x    - gy-gyoung 18 Sep 00:04 ├──  src
.rw-r--r--  121 gy-gyoung 17 Sep 20:55 │  ├──  main.module.scss
.rw-r--r--  490 gy-gyoung 18 Sep 00:04 │  └──  main.rs
.rw-r--r--    0 gy-gyoung 18 Sep 00:10 └──  Trunk.toml
```

- Trunk.toml
```toml
[serve]
# The address to serve on.
address = "127.0.0.1"
# The port to serve on.
port = 8000

[[hooks]]
stage = "pre_build"
command = "stylance"
command_arguments = [".", "--output-file", "index.css"]

[watch]
ignore = ["index.css"]

```

- Trunk.toml(stylance이렇게 하면 자동 실행 안됨..)
```toml
[serve]
# The address to serve on.
address = "127.0.0.1"
# The port to serve on.
port = 8000
```

# stylance 실행
```bash

stylance . --output-file ./index.css
Running stylance
././src/main.module.scss
```

- index.css
```css
<!DOCTYPE html>
<html>
    <head>
        <link data-trunk rel="css" href="index.css" />
    </head>
    <body></body>
</html>
```


# stylance-rs 세팅하기
- https://stackoverflow.com/questions/78244955/how-to-use-stylance-with-leptos

<hr>

# 다른거
  - https://lib.rs/crates/rcss-bundler

<hr>
