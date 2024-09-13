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

<hr>

- [백앤드-개발자가-알아야할-roadmap](#백앤드-개발자가-알아야할-roadmap)

<hr>

- [leptos setting](#leptosrust)
  - [하위폴더 특정파일 지우기(ex)target폴더 justfile같은거](#하위폴더-특정파일-지우기extarget폴더-justfile같은거)
  - [특정port찾아서 지우기_프로세스ID Kill하기](#특정port찾아서-지우기_프로세스id-kill하기)
- [VSCode 세팅](#vscode-settingsjson)
- [`trunk serve --open으로 작동시키기`](#cargo-install-cargo-generate)
  - [Trunk.toml로 내 맘대로 수정가능](#trunktoml로-내-맘대로-수정가능)

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

<hr>

- [암기해야할 주요 기능들](#암기해야할-주요-기능들)
  - [ReadSignal](https://docs.rs/leptos/latest/leptos/struct.ReadSignal.html)
  - [WriteSignal](https://docs.rs/leptos/latest/leptos/struct.WriteSignal.html)

<hr>

- Frontend
  - [Tailwind CSS](#tailwind-css)
    - [기본 폼Tailwind CSS](https://github.com/tailwindlabs/tailwindcss-forms) 
    - [Awesome-TailwindCSS](#awesome-tailwindcss)

<hr>

# Leptos(Rust)[|🔝|](#link)
https://book.leptos.dev/

- awesome-leptos 
  - https://github.com/leptos-rs/awesome-leptos

# cargo install cargo-generate[|🔝|](#link)

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

# Letos + Auxm[|🔝|](#link)
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

# Tailwind CSS[|🔝|](#link)

# Awesome-TailwindCSS[|🔝|](#link)
- https://github.com/aniftyco/awesome-tailwindcss

<hr>

<hr>
