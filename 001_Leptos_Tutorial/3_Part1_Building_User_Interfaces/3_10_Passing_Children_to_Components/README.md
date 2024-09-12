# Component Children

- It’s pretty common to want to pass children into a component, just as you can pass children into an HTML element. For example, imagine I have a <FancyForm/> component that enhances an HTML <form>. I need some way to pass all its inputs.

```rs
view! {
    <FancyForm>
        <fieldset>
            <label>
                "Some Input"
                <input type="text" name="something"/>
            </label>
        </fieldset>
        <button>"Submit"</button>
    </FancyForm>
}
```

- How can you do this in Leptos? There are basically two ways to pass components to other components:

- 1. render props: properties that are functions that return a view
  - 1. 소품 렌더링: 보기를 반환하는 함수인 속성
- 2. the children prop: a special component property that includes anything you pass as a child to the component.
  - 2. 어린이 소품: 어렸을 때 구성 요소에 물려준 모든 것을 포함하는 특별한 구성 요소 속성.

  
