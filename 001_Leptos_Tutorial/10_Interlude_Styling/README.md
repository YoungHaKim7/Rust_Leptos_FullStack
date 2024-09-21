- Leptos eBook
  - https://book.leptos.dev/interlude_styling.html
- stylance-rs
  - https://github.com/basro/stylance-rs

- SCSS
  - https://sass-lang.com/
  - https://sass-lang.com/documentation/

<hr>

# link

- [완전기초부터 stylance-rs세팅 알게 된 첫 출처 굿굿](#stylance-rs-세팅하기)
  - [SCSS_stylance파일구성 및 Install 및 전체 files구성](#scoped-css-style-imports-for-rust)
  - [stylance수동으로 실행하기_`Cargo.toml`에 설정하면 자동 실행도 됨](#stylance-실행)
    - [`Cargo.toml`설정하기 그냥 수동으로 하는게 더 나은거 같은데 아직 모름겠음..](#cargotoml세팅)
      - [Trunk_hook관련 Cargo.toml설정 설명서](https://trunkrs.dev/guide/build/hooks.html)
      - [Trunk(sass/scss, tailwind, css & icons 기타등등) 관련 설정 설명서](https://trunkrs.dev/assets/#css)
        - [위 내용과 동일 eBook형태](https://trunkrs.dev/guide/assets/index.html)

- [`trunk serve --open`할때 이미지 따로 cp수동으로 넣는거 귀찮아서 hook로 넣는방법 힌트](#trunk-serve---open할때-이미지-따로-cp수동으로-넣는거-귀찮아서-hook로-넣는방법-힌트)

<hr>

- [stylance말고 다른거 기타 등등etc](#rcss-bundler기타etc등등다른거) 

<hr>

# Scoped CSS style imports for rust.[|🔝|](#link)
- https://github.com/basro/stylance-rs

- Install stylance cli:

```
cargo install stylance-cli
```

```
cargo add stylance -F nightly
  
```

# (Stylance-rs)SCSS_Leptos플러그인files구성및 세팅[|🔝|](#link)

- dist & target은 만들어지는 폴더(styles.css로 변환되서 들어감)
```
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── Trunk.toml
├── dist
│   ├── a05_stylance_setting_final-7d4e3232bd20f3dd.js
│   ├── a05_stylance_setting_final-7d4e3232bd20f3dd_bg.wasm
│   ├── index.html
│   └── styles-aa32b28c4007bc9a.css
├── index.html
├── justfile
├── rust-toolchain.toml
├── src
│   ├── main.rs
│   └── scss
│       └── main.module.scss
├── styles
│   └── stylance
│       ├── _index.scss
│       └── main.module-c7311a6.scss
└── styles.css

6 directories, 16 files
```

- `Trunk.toml` (계속 리플레쉬되는 정신 없는 버젼ㅠㅠ) 
  - 이렇게 설정하면 `trunk serve --open` 하면 1초마다?? 계속 리플레쉬됨. 정신없음 ㅠㅠ

```toml
[serve]
# The address to serve on.
address = "127.0.0.1"
# The port to serve on.
port = 8000

[[hooks]]
stage = "pre_build"
command = "stylance"
command_arguments = [".", "--output-file", "styles.css"]

[watch]
ignore = ["styles.css"]

```

- Trunk.toml(stylance이렇게 하면 자동 실행 안됨..)
```toml
[serve]
# The address to serve on.
address = "127.0.0.1"
# The port to serve on.
port = 8000
```

# stylance 실행[|🔝|](#link)
```bash

stylance . --output-file ./styles.css
Running stylance
././src/main.module.scss

```

- Watch로 해서 실시간 체크하면서 하자
```
stylance --watch . --output-file ./styles.css
```

- `index.html`에서 `styles.css`(변환된 파일) 연결해줘야함
```css
<!DOCTYPE html>
<html>
    <head>
        <link data-trunk rel="css" href="styles.css" />
    </head>
    <body></body>
</html>
```

# Cargo.toml세팅[|🔝|](#link)
```toml
[package]
name = "a02_multi_class_scss_test"
version = "0.1.0"
edition = "2021"

[dependencies]
console_error_panic_hook = "0.1.7"
console_log = "1.0.0"
leptos = { version = "0.6.15", features = ["nightly", "csr"] }
log = "0.4.22"
stylance = { version = "0.5.1", features = ["nightly"] }

[package.metadata.stylance]
# output_file
# When set, stylance-cli will bundle all css module files
# into by concatenating them and put the result in this file.
output_file = "./styles/bundle.scss"

# output_dir
# When set, stylance-cli will create a folder named "stylance" inside
# the output_dir directory.
# The stylance folder will be populated with one file per detected css module
# and one _all.scss file that contains one `@use "file.module-hash.scss";` statement
# per module file.
# You can use that file to import all your modules into your main scss project.
output_dir = "./styles/"

# folders
# folders in which stylance cli will look for css module files.
# defaults to ["./src/"]
folders = ["./src/", "./styles/"]

# extensions
# files ending with these extensions will be considered to be
# css modules by stylance cli and will be included in the output
# bundle
# defaults to [".module.scss", ".module.css"]
extensions = [".module.scss", ".module.css"]

# scss_prelude
# When generating an scss file stylance-cli will prepend this string
# Useful to include a @use statement to all scss modules.
scss_prelude = '@use "../path/to/prelude" as *;'

# hash_len
# Controls how long the hash name used in scoped classes should be.
# It is safe to lower this as much as you want, stylance cli will produce an
# error if two files end up with colliding hashes.
# defaults to 7
hash_len = 7

# class_name_pattern
# Controls the shape of the transformed scoped class names.
# [name] will be replaced with the original class name
# [hash] will be replaced with the hash of css module file path.
# defaults to "[name]-[hash]"
class_name_pattern = "my-project-[name]-[hash]"

```

- Hash안하는거(뒤에 -  붙는거는 해결 못함 ㅠㅠ)
```toml

[package.metadata.stylance]
# output_file
# When set, stylance-cli will bundle all css module files
# into by concatenating them and put the result in this file.
output_file = "./styles/bundle.scss"

# output_dir
# When set, stylance-cli will create a folder named "stylance" inside
# the output_dir directory.
# The stylance folder will be populated with one file per detected css module
# and one _all.scss file that contains one `@use "file.module-hash.scss";` statement
# per module file.
# You can use that file to import all your modules into your main scss project.
output_dir = "./styles/"

# folders
# folders in which stylance cli will look for css module files.
# defaults to ["./src/"]
folders = ["./src/", "./styles/"]

# extensions
# files ending with these extensions will be considered to be
# css modules by stylance cli and will be included in the output
# bundle
# defaults to [".module.scss", ".module.css"]
extensions = [".module.scss", ".module.css"]

# scss_prelude
# When generating an scss file stylance-cli will prepend this string
# Useful to include a @use statement to all scss modules.
scss_prelude = '@use "../path/to/prelude" as *;'

# hash_len
# Controls how long the hash name used in scoped classes should be.
# It is safe to lower this as much as you want, stylance cli will produce an
# error if two files end up with colliding hashes.
# defaults to 7
hash_len = 0

# class_name_pattern
# Controls the shape of the transformed scoped class names.
# [name] will be replaced with the original class name
# [hash] will be replaced with the hash of css module file path.
# defaults to "[name]-[hash]"
# class_name_pattern = "my-project-[name]-[hash]"
```


# stylance-rs 세팅하기[|🔝|](#link)
- https://stackoverflow.com/questions/78244955/how-to-use-stylance-with-leptos

<hr>

# rcss-bundler기타etc..등등다른거[|🔝|](#link)
  - https://lib.rs/crates/rcss-bundler

<hr>

# `trunk serve --open`할때 이미지 따로 cp수동으로 넣는거 귀찮아서 hook로 넣는방법 힌트[|🔝|](#link)

- 테스트 예정
```
[[hooks]]
stage = "post_build"
command = "sh"
command_arguments = ["-c", "cp -rf imgs/ dist/. "]
```

- 출처 원본
```
[[hooks]]
stage = "post_build"
command = "sh"
command_arguments = ["-c", "cp -r ./dist/* $TRUNK_STAGING_DIR"]
```

출처 : https://github.com/trunk-rs/trunk/issues/9

<hr>
