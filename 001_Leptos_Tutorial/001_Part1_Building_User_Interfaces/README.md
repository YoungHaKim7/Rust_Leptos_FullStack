- https://book.leptos.dev/view/01_basic_component.html

# Iteration

- Whether you’re listing todos, displaying a table, or showing product images, iterating over a list of items is a common task in web applications. Reconciling the differences between changing sets of items can also be one of the trickiest tasks for a framework to handle well.

- Leptos supports two different patterns for iterating over items:

  - 1. For static views: `Vec<_>`
  - 2. For dynamic lists: `<For/>`

- 할 일을 나열하든, 표를 표시하든, 제품 이미지를 표시하든, 항목 목록을 반복하는 것은 웹 애플리케이션에서 흔히 볼 수 있는 작업입니다. 변경하는 항목 집합 간의 차이를 조정하는 것도 프레임워크가 잘 처리하기 위한 가장 까다로운 작업 중 하나가 될 수 있습니다.

- 렙토스는 항목 반복을 위한 두 가지 패턴을 지원합니다:


# Dynamic Rendering with the <For/> Component 
- 구성 요소를 사용한 동적 렌더링

- The <For/> component is a keyed dynamic list. It takes three props:
- <For/> 구성 요소는 키가 있는 동적 목록입니다. 세 가지 소품이 필요합니다:

  - `each`: a function (such as a signal) that returns the items `T` to be iterated over
    - 반복할 항목 T를 반환하는 함수(예: 신호)
  - `key`: a key function that takes `&T` and returns a stable, unique key or ID
    - `&T`를 사용하고 안정적이고 고유한 키 또는 ID를 반환하는 키 기능
  - `children`: renders each T into a view
    - 각 T를 보기로 렌더링합니다

- key is, well, the key. You can add, remove, and move items within the list. As long as each item’s key is stable over time, the framework does not need to rerender any of the items, unless they are new additions, and it can very efficiently add, remove, and move items as they change. This allows for extremely efficient updates to the list as it changes, with minimal additional work.
  - 키는 키입니다. 목록 내에서 항목을 추가, 제거 및 이동할 수 있습니다. 각 항목의 키가 시간이 지남에 따라 안정적이기만 하면 프레임워크는 새로운 추가 항목이 아닌 한 항목을 렌더링할 필요가 없으며, 변경 시 항목을 매우 효율적으로 추가, 제거 및 이동할 수 있습니다. 따라서 목록이 변경될 때 최소한의 추가 작업으로 매우 효율적으로 목록을 업데이트할 수 있습니다

- Creating a good key can be a little tricky. You generally do not want to use an index for this purpose, as it is not stable—if you remove or move items, their indices change.
  - 좋은 키를 만드는 것은 약간 까다로울 수 있습니다. 일반적으로 인덱스는 안정적이지 않으므로 항목을 제거하거나 이동하면 인덱스가 변경되므로 이 목적을 위해 인덱스를 사용하지 않는 것이 좋습니다.

- But it’s a great idea to do something like generating a unique ID for each row as it is generated, and using that as an ID for the key function.
  - 하지만 생성될 때 각 행에 고유한 ID를 생성하고 이를 키 기능의 ID로 사용하는 것과 같은 작업을 수행하는 것이 좋습니다.

Check out the <DynamicList/> component below for an example.
 
