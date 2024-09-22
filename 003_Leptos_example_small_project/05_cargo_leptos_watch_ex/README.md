# 실행

```bash
# leptos watch 실행
cargo leptos watch  


# 일단 수동으로 쓰자 상단에 @use 이거 지워야 된다. 왜 그런지 이유는 못찾음
stylance . --output-file ./styles.css


# watch 모드 로해야하는데 @use지워야 되서 일단 수동으로 해야함.
stylance --watch . --output-file ./styles.css
```
