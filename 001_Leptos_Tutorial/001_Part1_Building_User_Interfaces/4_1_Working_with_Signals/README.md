# Reactivity(반응성)

- Leptos is built on top of a fine-grained reactive system, designed to run expensive side effects (like rendering something in a browser, or making a network request) as infrequently as possible in response to change, reactive values.
  - 렙토스는 세분화된 반응형 시스템을 기반으로 구축되었으며, 브라우저에서 렌더링하거나 네트워크 요청과 같은 값비싼 부작용을 가능한 한 드물게 실행하도록 설계되었습니다.

- So far we’ve seen signals in action. These chapters will go into a bit more depth, and look at effects, which are the other half of the story.
  - 지금까지 우리는 행동하는 신호를 보았습니다. 이 챕터에서는 조금 더 자세히 설명하고 나머지 절반에 해당하는 효과를 살펴봅니다.

  
# Working with Signals

- So far we’ve used some simple examples of `create_signal`, which returns a `ReadSignal` getter and a `WriteSignal` setter.
Getting and Setting

- There are four basic signal operations:

- 1. `.get()` clones the current value of the signal and tracks any future changes to the value reactively.
  - 1. '.get ()'는 신호의 현재 값을 복제하고 향후 값의 변경 사항을 반응적으로 추적합니다.
- 2. `.with()` takes a function, which receives the current value of the signal by reference (&T), and tracks any future changes.
  - 2. '.with()'는 기준 신호의 현재 값(&T)을 수신하여 향후 변경 사항을 추적하는 함수를 사용합니다. 
- 3. `.set()` replaces the current value of the signal and notifies any subscribers that they need to update.
  - 3. '.set ()'는 신호의 현재 값을 대체하고 모든 가입자에게 업데이트가 필요하다고 알립니다
- 4. `.update()` takes a function, which receives a mutable reference to the current value of the signal (&mut T), and notifies any subscribers that they need to update. (.update() doesn’t return the value returned by the closure, but you can use .try_update() if you need to; for example, if you’re removing an item from a Vec<_> and want the removed item.)
  - 4. '.update()'는 신호(&mut T)의 현재 값에 대한 변경 가능한 참조를 수신하고 업데이트가 필요한 가입자에게 알림을 보내는 기능을 사용합니다. (.update()는 마감으로 반환된 값을 반환하지 않지만, 필요한 경우 .try_update()를 사용할 수 있습니다(예를 들어 Vec<_>에서 항목을 제거하고 제거된 항목을 원하는 경우
