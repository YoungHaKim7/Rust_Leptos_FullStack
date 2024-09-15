# Routing
- The Basics

- Routing drives most websites. A router is the answer to the question, “Given this URL, what should appear on the page?”
  - 라우팅은 대부분의 웹사이트를 구동합니다. 라우터는 "이 URL이 주어지면 페이지에 무엇이 표시되어야 하는가?"라는 질문에 대한 답입니다

- A URL consists of many parts. For example, the URL https://my-cool-blog.com/blog/search?q=Search#results consists of
  - - URL은 여러 부분으로 구성됩니다. 예를 들어, URL https://my-cool-blog.com/blog/search?q=Search#results 은 다음과 같이 구성됩니다

  - a scheme: `https`
  - a domain: `my-cool-blog.com`
  - a path: `/blog/search`
  - a query (or search): `?q=Search`
  - a hash: `#results`

- The Leptos Router works with the path and query (/blog/search?q=Search). Given this piece of the URL, what should the app render on the page?
  - 렙토스 라우터는 경로 및 쿼리(/blog/search?q=Search)와 함께 작동합니다. 이 URL 조각이 주어지면 앱은 페이지에서 무엇을 렌더링해야 하나요?
