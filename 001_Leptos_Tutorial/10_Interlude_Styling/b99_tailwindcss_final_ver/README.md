# 인스톨 & 실행하기(tailwindCSS)

- Trunk Tailwind CSS세팅 및 공식 사이트
  - https://trunkrs.dev/assets/#tailwind
  - https://tailwindcss.com/

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

- package.json

```json
{
  "scripts" : {
    "build" : "npx tailwindcss -i ./style/input.css -o ./style/output/output.css",
    "watch" : "npx tailwindcss -i ./style/input.css -o ./style/output/output.css --watch"
  },
  "devDependencies": {
    "tailwindcss": "^3.4.12"
  }
}
```

- tailwind.config.js

```js
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/*.rs", "./src/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

- style/input.css

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

<hr>

- index.html

```html
<!DOCTYPE html>
<html>
    <head>
        <link data-trunk rel="tailwind-css" href="style/input.css" />
    </head>
    <body></body>
</html>
```
