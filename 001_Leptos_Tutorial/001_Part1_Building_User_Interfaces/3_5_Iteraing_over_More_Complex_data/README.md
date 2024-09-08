- https://book.leptos.dev/view/04b_iteration.html

# You’ll notice a few differences here:
- 여기에서 몇 가지 차이점을 발견할 수 있습니다:

- we convert the data signal into an enumerated iterator
  - 데이터 신호를 열거된 반복기로 변환합니다
- we use the children prop explicitly, to make it easier to run some non-view code
  - 일부 비보기 코드를 더 쉽게 실행할 수 있도록 어린이 소품을 명시적으로 사용합니다
- we define a value memo and use that in the view. This value field doesn’t actually use the child being passed into each row. Instead, it uses the index and reaches back into the original data to get the value.
  - 값 메모를 정의하고 뷰에서 이를 사용합니다. 이 값 필드는 실제로 각 행에 전달되는 자식을 사용하지 않습니다. 대신 인덱스를 사용하여 원래 데이터에 다시 도달하여 값을 얻습니다.

- Every time data changes, now, each memo will be recalculated. If its value has changed, it will update its text node, without rerendering the whole row.
  - 데이터가 변경될 때마다 이제 각 메모가 다시 계산됩니다. 해당 메모의 값이 변경되면 전체 행을 다시 렌더링하지 않고 텍스트 노드를 업데이트합니다.

- Pros(장점)
  - We get the same fine-grained reactivity of the signal-wrapped version, without needing to wrap the data in signals.
    - 우리는 데이터를 신호로 감쌀 필요 없이 신호로 감싼 버전과 동일한 세분화된 반응성을 얻습니다.

- Cons(단점)
  - It’s a bit more complex to set up this memo-per-row inside the <For/> loop rather than using nested signals. For example, you’ll notice that we have to guard against the possibility that the data[index] would panic by using data.get(index), because this memo may be triggered to re-run once just after the row is removed. (This is because the memo for each row and the whole <For/> both depend on the same data signal, and the order of execution for multiple reactive values that depend on the same signal isn’t guaranteed.)
    - Note also that while memos memoize their reactive changes, the same calculation does need to re-run to check the value every time, so nested reactive signals will still be more efficient for pinpoint updates here.
  - 중첩 신호를 사용하는 것보다 <For/> 루프 내에서 행당 이 메모를 설정하는 것이 조금 더 복잡합니다. 예를 들어, 이 메모는 행이 제거된 직후에 한 번만 다시 실행되도록 트리거될 수 있으므로 data.get(index)을 사용하여 데이터[index]가 패닉에 빠질 가능성을 방지해야 합니다. (이는 각 행과 전체 <For/>의 메모가 모두 동일한 데이터 신호에 의존하며, 동일한 신호에 의존하는 여러 반응성 값에 대한 실행 순서가 보장되지 않기 때문입니다.)
  - 또한 메모는 반응성 변경 사항을 메모하지만 매번 값을 확인하려면 동일한 계산을 다시 실행해야 하므로 중첩된 반응 신호는 여기에서 정확하게 업데이트하는 데 여전히 더 효율적입니다.
