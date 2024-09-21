# class와 CSS 생성 공부

```html
class="bg-red-500 rounded text-white px-2 py-2"
````
- https://github.com/YoungHaKim7/Rust_Leptos_FullStack/commit/c2db5069ee2bbc7cc8e3cf5ad2040b2ee3603b8f#diff-a8c19186109b1e22f6a8c3f3ae794e7a96a459293f6d0a37bdfc87d563e5b861R40


```css
.rounded {
  border-radius: 0.25rem;
}
.bg-red-500 {
  --tw-bg-opacity: 1;
  background-color: rgb(239 68 68 / var(--tw-bg-opacity));
}
.px-2 {
  padding-left: 0.5rem;
  padding-right: 0.5rem;
}
.py-2 {
  padding-top: 0.5rem;
  padding-bottom: 0.5rem;
}
.text-white {
  --tw-text-opacity: 1;
  color: rgb(255 255 255 / var(--tw-text-opacity));
}
    
```

<hr>

<hr>

```html

<div class="w-full max-w-[64rem] mx-auto items-center justify-center align-center">
    "Home Page here"
</div>
```

- https://github.com/YoungHaKim7/Rust_Leptos_FullStack/commit/4653a148898098f28e80b3df1b856e14131a83d7

```css
.w-full {
  width: 100%;
}
.max-w-\[64rem\] {
  max-width: 64rem;
}
.items-center {
  align-items: center;
}
.justify-center {
  justify-content: center;
}
.rounded {
  border-radius: 0.25rem;
}

.bg-gray-900 {
  --tw-bg-opacity: 1;
  background-color: rgb(17 24 39 / var(--tw-bg-opacity));
}
.bg-red-500 {
  --tw-bg-opacity: 1;
  background-color: rgb(239 68 68 / var(--tw-bg-opacity));
}

```


<hr>

```html
<div class="mt-20">
    <div class="text-white flex flex-col w-full mx-auto items-center justify-center z-25">
        <div class="flex flex-row flex-col w-full mx-auto items-center justify-center">
            <div class="flex flex-row w-full max-w-[52rem]">
                <div class="pr-4 mt-4 text-xl">
                    "Members"
                </div>
                <hr class="w-full max-w-[48rem] pl-4 pr-4 pt-4 mt-8 mr-4" />
                <button on:click=on_click class=ADD_BUTTON_STYLE>
                    "Add"
                </button>
            </div>
        </div>
    </div>
</div>
```

- https://github.com/YoungHaKim7/Rust_Leptos_FullStack/commit/a05b47c815e9b88fdf739256188f226b02eda63d#diff-0a65ae8763842db733c2486ab9185020989e2be0b56e756a826e1caea9047413R17

```css
.mx-auto {
  margin-left: auto;
  margin-right: auto;
}

.ml-4 {
  margin-left: 1rem;
}

.mr-4 {
  margin-right: 1rem;
}

.mt-20 {
  margin-top: 5rem;
}
.mt-4 {
  margin-top: 1rem;
}
.mt-8 {
  margin-top: 2rem;
}


.w-full {
  width: 100%;
}

.max-w-\[64rem\] {
  max-width: 64rem;
}

.max-w-\[52rem\] {
  max-width: 52rem;
}

.max-w-\[48rem\] {
  max-width: 48rem;
}
.flex-row {
  flex-direction: row;
}

.flex-col {
  flex-direction: column;
}
.items-center {
  align-items: center;
}



.rounded {
  border-radius: 0.25rem;
}
.border-b-0 {
  border-bottom-width: 0px;
}

.border-b-2 {
  border-bottom-width: 2px;
}

.border-\[\#7734e7\] {
  --tw-border-opacity: 1;
  border-color: rgb(119 52 231 / var(--tw-border-opacity));
}

.border-\[\#9734e7\] {
  --tw-border-opacity: 1;
  border-color: rgb(151 52 231 / var(--tw-border-opacity));
}

.bg-gray-900 {
  --tw-bg-opacity: 1;
  background-color: rgb(17 24 39 / var(--tw-bg-opacity));
}

.bg-gray-500 {
  --tw-bg-opacity: 1;
  background-color: rgb(107 114 128 / var(--tw-bg-opacity));
}

.bg-\[\#7734e7\] {
  --tw-bg-opacity: 1;
  background-color: rgb(119 52 231 / var(--tw-bg-opacity));
}
.px-20 {
  padding-left: 5rem;
  padding-right: 5rem;
}

.px-8 {
  padding-left: 2rem;
  padding-right: 2rem;
}
.py-2 {
  padding-top: 0.5rem;
  padding-bottom: 0.5rem;
}
.pt-8 {
  padding-top: 2rem;
}

.pl-4 {
  padding-left: 1rem;
}
.pr-4 {
  padding-right: 1rem;
}
.pt-4 {
  padding-top: 1rem;
}
.text-xl {
  font-size: 1.25rem;
  line-height: 1.75rem;
}
.text-white {
  --tw-text-opacity: 1;
  color: rgb(255 255 255 / var(--tw-text-opacity));
}

.transition-all {
  transition-property: all;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;
}
.duration-1000 {
  transition-duration: 1000ms;
}
.ease-in-out {
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
}
.hover\:border-b-2:hover {
  border-bottom-width: 2px;
}
.hover\:bg-\[\#8448e9\]:hover {
  --tw-bg-opacity: 1;
  background-color: rgb(132 72 233 / var(--tw-bg-opacity));
}


```


<hr>

```html
const INPUT_STYLE: &str = "w-full h-12 bg-[#333333] pr-r pl-6 py-4 text-white mt-6 outline-none focus:outline-none focus:pl-7 transition-all duration-1000 ease-in-out";
const CANCEL_BUTTON_STYLE: &str = "mt-10 bg-[#555555] px-8 py-2 rounded text-white mr-3 transition-all duration-1000 ease-in-out hover:bg-[#666666]";
const ADD_BUTTON_STYLE: &str = "mt-10 bg-[#7734e7] px-8 py-2 rounded text-white transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";
const NO_ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#7734e7] px-6 pt-5 h-[32rem] w-full max-w-[36rem] z-50 -mt-2 fixed z-50";
const ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#7734e7] px-6 pt-5 h-[32rem] w-full max-w-[36rem] z-50 -mt-2 fixed z-50";


<div class="flex flex-col w-full h-full z-50 mx-auto items-center align-center">
    <div class={move || {
        if if_error() { ERROR_STYLE }
        else { NO_ERROR_STYLE }
    }}>
      <Show when=move || { if_error() }>
          <p class="text-white bg-red-500 rounded w-full h-12 px-5 py-3 transition-all duration-750 ease-in-out">
              { error_message() }
          </p>
      </Show>
      <p class="text-white pt-5">"Add New Employee"</p>
    </div>
</div>

<input class="flex flex-row w-full items-right justify-right"/>
    <button on:click=on_close class=CANCEL_BUTTON_STYLE>
        "Cancel"
    </button>
    <button on:click=on_click class=ADD_BUTTON_STYLE>
        "Add"
    </button>


```

- https://github.com/YoungHaKim7/Rust_Leptos_FullStack/commit/fbfa57c522aca9fc72fd6b2f16bc719cbd640c83


```css

.z-50 {
  z-index: 50;
}

.mt-6 {
  margin-top: 1.5rem;
}
.mr-3 {
  margin-right: 0.75rem;
}
.mt-10 {
  margin-top: 2.5rem;
}
.-mt-2 {
  margin-top: -0.5rem;
}


.h-\[29rem\] {
  height: 29rem;
}
.h-\[32rem\] {
  height: 32rem;
}
.h-full {
  height: 100%;
}
.w-full {
  width: 100%;
}



.max-w-\[36rem\] {
  max-width: 36rem;
}
.flex-row {
  flex-direction: row;
}


.border-t-8 {
  border-top-width: 8px;
}
.border-\[\#7734e7\] {
  --tw-border-opacity: 1;
  border-color: rgb(119 52 231 / var(--tw-border-opacity));
}



.bg-\[\#333333\] {
  --tw-bg-opacity: 1;
  background-color: rgb(51 51 51 / var(--tw-bg-opacity));
}
.bg-\[\#555555\] {
  --tw-bg-opacity: 1;
  background-color: rgb(85 85 85 / var(--tw-bg-opacity));
}
.bg-\[\#222222\] {
  --tw-bg-opacity: 1;
  background-color: rgb(34 34 34 / var(--tw-bg-opacity));
}
.bg-red-500 {
  --tw-bg-opacity: 1;
  background-color: rgb(239 68 68 / var(--tw-bg-opacity));
}

.py-4 {
  padding-top: 1rem;
  padding-bottom: 1rem;
}
.px-6 {
  padding-left: 1.5rem;
  padding-right: 1.5rem;
}
.px-5 {
  padding-left: 1.25rem;
  padding-right: 1.25rem;
}
.py-3 {
  padding-top: 0.75rem;
  padding-bottom: 0.75rem;
}
.pt-8 {
  padding-top: 2rem;
}



.pl-6 {
  padding-left: 1.5rem;
}
.pt-5 {
  padding-top: 1.25rem;
}
.text-xl {
  font-size: 1.25rem;
  line-height: 1.75rem;
}

.text-white {
  --tw-text-opacity: 1;
  color: rgb(255 255 255 / var(--tw-text-opacity));
}

.outline-none {
  outline: 2px solid transparent;
  outline-offset: 2px;
}
.transition-all {
  transition-property: all;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;
}


.duration-1000 {
  transition-duration: 1000ms;
}

.ease-in-out {
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
}

.hover\:border-b-2:hover {
  border-bottom-width: 2px;
}

.hover\:bg-\[\#8448e9\]:hover {
  --tw-bg-opacity: 1;
  background-color: rgb(132 72 233 / var(--tw-bg-opacity));
}


.hover\:bg-\[\#666666\]:hover {
  --tw-bg-opacity: 1;
  background-color: rgb(102 102 102 / var(--tw-bg-opacity));
}
.focus\:pl-7:focus {
  padding-left: 1.75rem;
}
.focus\:outline-none:focus {
  outline: 2px solid transparent;
  outline-offset: 2px;
}

```


<hr>


<hr>
