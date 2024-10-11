<p align="center">
  <img width=40px src="https://user-images.githubusercontent.com/67513038/213436632-820a1675-98d9-4626-979d-be63c60cdcb7.png"/>
  <img src="https://github.com/user-attachments/assets/4a2a5554-098a-41d4-8c53-68e3c0ebe408" />
</p>

# Welcome to Leptos!
- Leptos is a framework for building full-stack web applications with Rust.

<hr>

# link

- [WASM우수한 성능에 대해The Truth about Rust/WebAssembly Performance |Greg Johnsto](https://youtu.be/4KtotxNAwME?si=9MefecdXc_ppF0rM)
  - [성능 비교하는 사이트](https://krausest.github.io/js-framework-benchmark/2022/table_chrome_103.0.5060.53_osx.html)
    - [leptos-rs생산적인 토론을 하는 곳](https://github.com/leptos-rs/leptos/discussions/125)

<hr>

- [백앤드-개발자가-알아야할-roadmap](#백앤드-개발자가-알아야할-roadmap)
- [Complete Backend Software Engineer Mind Map - Everything You Need to Know (2 HOURS!)Caleb Curry](https://youtu.be/oVfw8Oj-uH8?si=WcGC1kkwRM2c6csz)

<hr>

- [leptos setting](#leptosrust)
  - [하위폴더 특정파일 지우기(ex)target폴더 justfile같은거](#하위폴더-특정파일-지우기extarget폴더-justfile같은거)
  - [특정port찾아서 지우기_프로세스ID Kill하기](#특정port찾아서-지우기_프로세스id-kill하기)
- [VSCode 세팅](#vscode-settingsjson)
- [`trunk serve --open으로 작동시키기`](#cargo-install-cargo-generate)
  - [Trunk.toml로 내 맘대로 수정가능](#trunktoml로-내-맘대로-수정가능)

<hr>

- 외국 영상 Leptos관련
  - [Leptos in Five Minutes | Greg Johnston](https://youtu.be/K_TmEPAD9Ig?si=JPpNQaf_ap5mXYdt)

<hr>

- Rust Leptos공식 Tutorial영상
  - [영상모아보기 Full-Stack Web Apps with Rust and Leptos - July 3, 2024 | Training 4 Programmers](https://youtube.com/playlist?list=PL40umFWNzu2UGL35Qp68s3uP5enQwmwPi&si=5NQdv6PY7sj9fV4y)


<hr>

- CSS & SCSS 세팅관련
  - stylance 세팅(난 이게 더 좋다. 러스트로 만든거라 ㅋ)
    - [stylance-rs세팅 example](./001_Leptos_Tutorial/10_Interlude_Styling/z01_no_hash_test)
    ```bash
    # stylance watch css
    stylance --watch . --output-file ./styles.css
    ``` 
    - `cargo leptos watch`로
      - [SCSS + Stylance-rs _ Image까지 나오는 조합 굿 굿](./003_Leptos_example_small_project/05_cargo_leptos_watch_ex/a01-cargo-leptos-watch-ex)
  - tailwindCSS 세팅 
    - [tailwindcss세팅 example](./001_Leptos_Tutorial/10_Interlude_Styling/b99_tailwindcss_final_ver)
    ```bash
    # tailwindcss watch css
    # package.json & tailwind.config.js 안에 작성필요
    npm run watch
    ```
    - install
    ```bash
    npm install -D tailwindcss
    npx tailwindcss init
    ```

    - run
    ```
    trunk serve --open
    
    npm run watch
    ```


<hr>

- Leptos최신소식
  - [(24. 9. 29)Leptos September Meetup: Cloudflare and Leptos, 0.7 Update, and More | Leptos](https://www.youtube.com/live/EsBLdHFPL1g?si=RF-_6b9XhduZlui7)

<hr />


- Leptos 외국 관련글모음
  - [242402_Full Stack Rust with Leptos](https://benw.is/posts/full-stack-rust-with-leptos) 

<hr>

# Rust_Leptos_FullStack
- Leptos
  - https://leptos.dev/

- Leptos eBook
  - https://book.leptos.dev/

- Leptos API문서
  - https://docs.rs/leptos/latest/leptos/

- Leptos Github
  - https://github.com/leptos-rs/leptos
    - Leptos example
      - https://github.com/leptos-rs/leptos/tree/main/examples

- Leptos-Use Guide(Collection of essential Leptos utilities)
  - https://leptos-use.rs/
    - https://github.com/synphonyte/leptos-use

<hr>

- [암기해야할 주요 기능들](#암기해야할-주요-기능들)
  - [ReadSignal](https://docs.rs/leptos/latest/leptos/struct.ReadSignal.html)
  - [WriteSignal](https://docs.rs/leptos/latest/leptos/struct.WriteSignal.html)

<hr>

- Frontend
  - [SCSS_stylance-rs 세팅하기](#scoped-css-style-imports-for-rust)
    - [SCSS + Stylance-rs + RustLeptos조합](./001_Leptos_Tutorial/10_Interlude_Styling) 
       - [SCSS + Stylance-rs _ Image까지 나오는 조합 굿 굿](./003_Leptos_example_small_project/05_cargo_leptos_watch_ex/a01-cargo-leptos-watch-ex)
  - [Tailwind CSS](#tailwind-css)
    - [기본 폼Tailwind CSS](https://github.com/tailwindlabs/tailwindcss-forms) 
    - [Awesome-TailwindCSS](#awesome-tailwindcss)

<hr>

# Leptos(Rust)[|🔝|](#link)
https://book.leptos.dev/

- awesome-leptos 
  - https://github.com/leptos-rs/awesome-leptos

# cargo install cargo-generate[|🔝|](#link)

- https://github.com/trunk-rs/trunk
  
```
trunk serve --port 3000 --open

# or
trunk serve --open
```

# Trunk.toml로 내 맘대로 수정가능[|🔝|](#link)

```toml
[serve]
# The address to serve on.
address = "127.0.0.1"
# The port to serve on.
port = 8000
```

# 특정port찾아서 지우기_프로세스ID Kill하기[|🔝|](#link)

```
# 포트찾기
lsof -i :8080

lsof -i :"찾고 싶은 포트번호"

# 죽이고 싶은 포트 죽이기(맨 뒤는 포트 No.쓰면됨)
kill -9 52474

kill -9 "PID 프로세스 아디 쓰면됨"
```

- port & process kill다른 방법
  - [Linux] 특정 포트를 사용하는 프로세스 확인 - https://hbase.tistory.com/m/227

# 하위폴더 특정파일 지우기(ex)target폴더 justfile같은거[|🔝|](#link)

```bash
$ find . -type f -name "justfile" -exec rm {} \;

```
- https://ccambo.tistory.com/entry/MacOS-%ED%8A%B9%EC%A0%95-%EA%B2%BD%EB%A1%9C-%EB%B0%91%EC%9D%98-%EB%94%94%EB%A0%89%ED%84%B0%EB%A6%AC-%EC%9D%BC%EA%B4%84-%EC%82%AD%EC%A0%9C%ED%95%98%EA%B8%B0

# VSCode `settings.json`:[|🔝|](#link)
- https://book.leptos.dev/getting_started/leptos_dx.html
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



<hr>

# Integrating Leptos, Rust's Modern Web Development Framework, into my uptime tracker | coreyja[|🔝|](#link)

- https://youtu.be/wm7BRRUdDt4?si=J7ItFNF9W63-zKZf

<hr>

# Letos + Axum[|🔝|](#link)
- https://github.com/leptos-rs/start-axum

<hr>

# Yew + Axum or Leptos?[|🔝|](#link)
- Moderately new to Rust, and have a project in mind of a real-time markdown editor. For now, I just want to be able to have multiple users be able to edit a markdown document, and have the changes saved to a markdown file on the server (kinda like Google Docs)
  - Trying to decide which of the two makes the most sense, or if I should use Rust on the frontend at all!
- https://www.reddit.com/r/rust/s/8yDbt0gjmc

# 백앤드 개발자가 알아야할 RoadMap[|🔝|](#link)

![KakaoTalk_20240129_160102026](https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/8473d4f1-1428-4b6f-b32d-ca926d4d80e9)

- 러스트 개발자 로드맵 Rust dev
  - https://roadmap.sh/rust
 
<hr>

# CSS & SCSS[|🔝|](#link)
- Rust Leptos 설명서
  - https://book.leptos.dev/interlude_styling.html
- SCSS 기초
  - https://sass-lang.com/

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


# stylance-rs 세팅하기
- https://stackoverflow.com/questions/78244955/how-to-use-stylance-with-leptos

- 다른거
  - https://lib.rs/crates/rcss-bundler

# stylance 치면 scss -> css로 변환됨.

```bash
# 1회성 수동모드
stylance . --output-file ./index.css
Running stylance
././src/main.module.scss


# watch 모드(실시간으로 확인하면서 하자)
stylance --watch . --output-file ./styles.css
```


<hr>

# px to rem converter[|🔝|](#link)

- https://nekocalc.com/px-to-rem-converter

<hr>

# Tailwind CSS[|🔝|](#link)

# Awesome-TailwindCSS[|🔝|](#link)
- https://github.com/aniftyco/awesome-tailwindcss

<hr>

<hr>
