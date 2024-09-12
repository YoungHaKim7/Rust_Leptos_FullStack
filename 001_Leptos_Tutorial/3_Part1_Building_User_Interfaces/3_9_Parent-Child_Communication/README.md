- https://book.leptos.dev/view/08_parent_child.html

<hr>


# WriteSignal과 Callback은 TradeOff 관계

- You’ll notice that whereas `<ButtonA/>` was given a WriteSignal and decided how to mutate it, `<ButtonB/>` simply fires an event: the mutation happens back in <App/>. This has the advantage of keeping local state local, preventing the problem of spaghetti mutation. But it also means the logic to mutate that signal needs to exist up in `<App/>`, not down in `<ButtonB/>`. These are real trade-offs, not a simple right-or-wrong choice.

- Note the way we use the `Callback<In, Out>` type. This is basically a wrapper around a closure `Fn(In) -> Out` that is also Copy and makes it easy to pass around.

- We also used the `#[prop(into)]` attribute so we can pass a normal closure into on_click. Please see the chapter "into Props" for more details.

- TradeOff관계 이해

- `<ButtonA/>`에 라이트시그널을 부여하고 돌연변이 방법을 결정한 반면, `<ButtonB/>`는 단순히 이벤트를 발생시킵니다: 돌연변이는 `<App/>`에서 다시 발생합니다. 이는 지역 상태를 로컬로 유지하여 스파게티 돌연변이 문제를 방지할 수 있다는 장점이 있습니다. 하지만 이는 또한 해당 신호를 돌연변이시키는 논리가 `<ButtonB/>`에서 다운되지 않고 `<App/>`에서 존재해야 한다는 것을 의미합니다. 이는 단순한 옳고 그름의 선택이 아니라 실제 트레이드오프입니다.

- `Callback<In, Out>` 유형을 사용하는 방식에 주목하세요. 이는 기본적으로 클로저 `Fn(In) -> Out` 주변의 포장지이며, 복사이며 쉽게 전달할 수 있습니다.

- 또한 일반적인 클로저를 on_click에 전달할 수 있도록 `#[prop(into)]` 속성을 사용했습니다. 자세한 내용은 "소품 안으로" 장을 참조하세요.

# 참고 3.3.3 Components and Props
- https://book.leptos.dev/view/03_components.html#into-props

# 3.3.9(Parent-Child Communication)
- https://book.leptos.dev/view/08_parent_child.html

There are four basic patterns of parent-child communication in Leptos.
- 1. Pass a `WriteSignal`
- 2. Use a Callback
  - 2.1 Use Closure instead of Callback
- 3. Use an Event Listener
- 4. Providing a Context
  - 4.1 The Context API
