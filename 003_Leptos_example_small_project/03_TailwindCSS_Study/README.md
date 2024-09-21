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

<hr/>

```html
<figure class="md:flex bg-slate-100 rounded-xl p-8 md:p-0 dark:bg-slate-800">
  <img class="w-24 h-24 md:w-48 md:h-auto md:rounded-none rounded-full mx-auto" src="test.jpg" alt="" width="384" height="512"/>
  <div class="pt-6 md:p-8 text-center md:text-left space-y-4">
    <blockquote>
      <p class="text-lg font-medium text-sky-100">
        "Tailwind CSS is the only framework that I've seen scale on large teams. It's easy to customize, adapts to any design, and the build size is tiny."
      </p>
    </blockquote>
    <figcaption class="font-medium">
      <div class="text-sky-500 dark:text-sky-400">
        Gyoung Taildwindcss test
      </div>
      <div class="text-slate-700 dark:text-slate-500">
        YouTuber
      </div>
    </figcaption>
  </div>
</figure>
```

```css
*, ::before, ::after {
  --tw-border-spacing-x: 0;
  --tw-border-spacing-y: 0;
  --tw-translate-x: 0;
  --tw-translate-y: 0;
  --tw-rotate: 0;
  --tw-skew-x: 0;
  --tw-skew-y: 0;
  --tw-scale-x: 1;
  --tw-scale-y: 1;
  --tw-pan-x:  ;
  --tw-pan-y:  ;
  --tw-pinch-zoom:  ;
  --tw-scroll-snap-strictness: proximity;
  --tw-gradient-from-position:  ;
  --tw-gradient-via-position:  ;
  --tw-gradient-to-position:  ;
  --tw-ordinal:  ;
  --tw-slashed-zero:  ;
  --tw-numeric-figure:  ;
  --tw-numeric-spacing:  ;
  --tw-numeric-fraction:  ;
  --tw-ring-inset:  ;
  --tw-ring-offset-width: 0px;
  --tw-ring-offset-color: #fff;
  --tw-ring-color: rgb(59 130 246 / 0.5);
  --tw-ring-offset-shadow: 0 0 #0000;
  --tw-ring-shadow: 0 0 #0000;
  --tw-shadow: 0 0 #0000;
  --tw-shadow-colored: 0 0 #0000;
  --tw-blur:  ;
  --tw-brightness:  ;
  --tw-contrast:  ;
  --tw-grayscale:  ;
  --tw-hue-rotate:  ;
  --tw-invert:  ;
  --tw-saturate:  ;
  --tw-sepia:  ;
  --tw-drop-shadow:  ;
  --tw-backdrop-blur:  ;
  --tw-backdrop-brightness:  ;
  --tw-backdrop-contrast:  ;
  --tw-backdrop-grayscale:  ;
  --tw-backdrop-hue-rotate:  ;
  --tw-backdrop-invert:  ;
  --tw-backdrop-opacity:  ;
  --tw-backdrop-saturate:  ;
  --tw-backdrop-sepia:  ;
  --tw-contain-size:  ;
  --tw-contain-layout:  ;
  --tw-contain-paint:  ;
  --tw-contain-style:  ;
}

::backdrop {
  --tw-border-spacing-x: 0;
  --tw-border-spacing-y: 0;
  --tw-translate-x: 0;
  --tw-translate-y: 0;
  --tw-rotate: 0;
  --tw-skew-x: 0;
  --tw-skew-y: 0;
  --tw-scale-x: 1;
  --tw-scale-y: 1;
  --tw-pan-x:  ;
  --tw-pan-y:  ;
  --tw-pinch-zoom:  ;
  --tw-scroll-snap-strictness: proximity;
  --tw-gradient-from-position:  ;
  --tw-gradient-via-position:  ;
  --tw-gradient-to-position:  ;
  --tw-ordinal:  ;
  --tw-slashed-zero:  ;
  --tw-numeric-figure:  ;
  --tw-numeric-spacing:  ;
  --tw-numeric-fraction:  ;
  --tw-ring-inset:  ;
  --tw-ring-offset-width: 0px;
  --tw-ring-offset-color: #fff;
  --tw-ring-color: rgb(59 130 246 / 0.5);
  --tw-ring-offset-shadow: 0 0 #0000;
  --tw-ring-shadow: 0 0 #0000;
  --tw-shadow: 0 0 #0000;
  --tw-shadow-colored: 0 0 #0000;
  --tw-blur:  ;
  --tw-brightness:  ;
  --tw-contrast:  ;
  --tw-grayscale:  ;
  --tw-hue-rotate:  ;
  --tw-invert:  ;
  --tw-saturate:  ;
  --tw-sepia:  ;
  --tw-drop-shadow:  ;
  --tw-backdrop-blur:  ;
  --tw-backdrop-brightness:  ;
  --tw-backdrop-contrast:  ;
  --tw-backdrop-grayscale:  ;
  --tw-backdrop-hue-rotate:  ;
  --tw-backdrop-invert:  ;
  --tw-backdrop-opacity:  ;
  --tw-backdrop-saturate:  ;
  --tw-backdrop-sepia:  ;
  --tw-contain-size:  ;
  --tw-contain-layout:  ;
  --tw-contain-paint:  ;
  --tw-contain-style:  ;
}

/*
! tailwindcss v3.4.12 | MIT License | https://tailwindcss.com
*/

/*
1. Prevent padding and border from affecting element width. (https://github.com/mozdevs/cssremedy/issues/4)
2. Allow adding a border to an element by just adding a border-width. (https://github.com/tailwindcss/tailwindcss/pull/116)
*/

*,
::before,
::after {
  box-sizing: border-box;
  /* 1 */
  border-width: 0;
  /* 2 */
  border-style: solid;
  /* 2 */
  border-color: #e5e7eb;
  /* 2 */
}

::before,
::after {
  --tw-content: '';
}

/*
1. Use a consistent sensible line-height in all browsers.
2. Prevent adjustments of font size after orientation changes in iOS.
3. Use a more readable tab size.
4. Use the user's configured `sans` font-family by default.
5. Use the user's configured `sans` font-feature-settings by default.
6. Use the user's configured `sans` font-variation-settings by default.
7. Disable tap highlights on iOS
*/

html,
:host {
  line-height: 1.5;
  /* 1 */
  -webkit-text-size-adjust: 100%;
  /* 2 */
  -moz-tab-size: 4;
  /* 3 */
  -o-tab-size: 4;
     tab-size: 4;
  /* 3 */
  font-family: ui-sans-serif, system-ui, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
  /* 4 */
  font-feature-settings: normal;
  /* 5 */
  font-variation-settings: normal;
  /* 6 */
  -webkit-tap-highlight-color: transparent;
  /* 7 */
}

/*
1. Remove the margin in all browsers.
2. Inherit line-height from `html` so users can set them as a class directly on the `html` element.
*/

body {
  margin: 0;
  /* 1 */
  line-height: inherit;
  /* 2 */
}

/*
1. Add the correct height in Firefox.
2. Correct the inheritance of border color in Firefox. (https://bugzilla.mozilla.org/show_bug.cgi?id=190655)
3. Ensure horizontal rules are visible by default.
*/

hr {
  height: 0;
  /* 1 */
  color: inherit;
  /* 2 */
  border-top-width: 1px;
  /* 3 */
}

/*
Add the correct text decoration in Chrome, Edge, and Safari.
*/

abbr:where([title]) {
  -webkit-text-decoration: underline dotted;
          text-decoration: underline dotted;
}

/*
Remove the default font size and weight for headings.
*/

h1,
h2,
h3,
h4,
h5,
h6 {
  font-size: inherit;
  font-weight: inherit;
}

/*
Reset links to optimize for opt-in styling instead of opt-out.
*/

a {
  color: inherit;
  text-decoration: inherit;
}

/*
Add the correct font weight in Edge and Safari.
*/

b,
strong {
  font-weight: bolder;
}

/*
1. Use the user's configured `mono` font-family by default.
2. Use the user's configured `mono` font-feature-settings by default.
3. Use the user's configured `mono` font-variation-settings by default.
4. Correct the odd `em` font sizing in all browsers.
*/

code,
kbd,
samp,
pre {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  /* 1 */
  font-feature-settings: normal;
  /* 2 */
  font-variation-settings: normal;
  /* 3 */
  font-size: 1em;
  /* 4 */
}

/*
Add the correct font size in all browsers.
*/

small {
  font-size: 80%;
}

/*
Prevent `sub` and `sup` elements from affecting the line height in all browsers.
*/

sub,
sup {
  font-size: 75%;
  line-height: 0;
  position: relative;
  vertical-align: baseline;
}

sub {
  bottom: -0.25em;
}

sup {
  top: -0.5em;
}

/*
1. Remove text indentation from table contents in Chrome and Safari. (https://bugs.chromium.org/p/chromium/issues/detail?id=999088, https://bugs.webkit.org/show_bug.cgi?id=201297)
2. Correct table border color inheritance in all Chrome and Safari. (https://bugs.chromium.org/p/chromium/issues/detail?id=935729, https://bugs.webkit.org/show_bug.cgi?id=195016)
3. Remove gaps between table borders by default.
*/

table {
  text-indent: 0;
  /* 1 */
  border-color: inherit;
  /* 2 */
  border-collapse: collapse;
  /* 3 */
}

/*
1. Change the font styles in all browsers.
2. Remove the margin in Firefox and Safari.
3. Remove default padding in all browsers.
*/

button,
input,
optgroup,
select,
textarea {
  font-family: inherit;
  /* 1 */
  font-feature-settings: inherit;
  /* 1 */
  font-variation-settings: inherit;
  /* 1 */
  font-size: 100%;
  /* 1 */
  font-weight: inherit;
  /* 1 */
  line-height: inherit;
  /* 1 */
  letter-spacing: inherit;
  /* 1 */
  color: inherit;
  /* 1 */
  margin: 0;
  /* 2 */
  padding: 0;
  /* 3 */
}

/*
Remove the inheritance of text transform in Edge and Firefox.
*/

button,
select {
  text-transform: none;
}

/*
1. Correct the inability to style clickable types in iOS and Safari.
2. Remove default button styles.
*/

button,
input:where([type='button']),
input:where([type='reset']),
input:where([type='submit']) {
  -webkit-appearance: button;
  /* 1 */
  background-color: transparent;
  /* 2 */
  background-image: none;
  /* 2 */
}

/*
Use the modern Firefox focus style for all focusable elements.
*/

:-moz-focusring {
  outline: auto;
}

/*
Remove the additional `:invalid` styles in Firefox. (https://github.com/mozilla/gecko-dev/blob/2f9eacd9d3d995c937b4251a5557d95d494c9be1/layout/style/res/forms.css#L728-L737)
*/

:-moz-ui-invalid {
  box-shadow: none;
}

/*
Add the correct vertical alignment in Chrome and Firefox.
*/

progress {
  vertical-align: baseline;
}

/*
Correct the cursor style of increment and decrement buttons in Safari.
*/

::-webkit-inner-spin-button,
::-webkit-outer-spin-button {
  height: auto;
}

/*
1. Correct the odd appearance in Chrome and Safari.
2. Correct the outline style in Safari.
*/

[type='search'] {
  -webkit-appearance: textfield;
  /* 1 */
  outline-offset: -2px;
  /* 2 */
}

/*
Remove the inner padding in Chrome and Safari on macOS.
*/

::-webkit-search-decoration {
  -webkit-appearance: none;
}

/*
1. Correct the inability to style clickable types in iOS and Safari.
2. Change font properties to `inherit` in Safari.
*/

::-webkit-file-upload-button {
  -webkit-appearance: button;
  /* 1 */
  font: inherit;
  /* 2 */
}

/*
Add the correct display in Chrome and Safari.
*/

summary {
  display: list-item;
}

/*
Removes the default spacing and border for appropriate elements.
*/

blockquote,
dl,
dd,
h1,
h2,
h3,
h4,
h5,
h6,
hr,
figure,
p,
pre {
  margin: 0;
}

fieldset {
  margin: 0;
  padding: 0;
}

legend {
  padding: 0;
}

ol,
ul,
menu {
  list-style: none;
  margin: 0;
  padding: 0;
}

/*
Reset default styling for dialogs.
*/

dialog {
  padding: 0;
}

/*
Prevent resizing textareas horizontally by default.
*/

textarea {
  resize: vertical;
}

/*
1. Reset the default placeholder opacity in Firefox. (https://github.com/tailwindlabs/tailwindcss/issues/3300)
2. Set the default placeholder color to the user's configured gray 400 color.
*/

input::-moz-placeholder, textarea::-moz-placeholder {
  opacity: 1;
  /* 1 */
  color: #9ca3af;
  /* 2 */
}

input::placeholder,
textarea::placeholder {
  opacity: 1;
  /* 1 */
  color: #9ca3af;
  /* 2 */
}

/*
Set the default cursor for buttons.
*/

button,
[role="button"] {
  cursor: pointer;
}

/*
Make sure disabled buttons don't get the pointer cursor.
*/

:disabled {
  cursor: default;
}

/*
1. Make replaced elements `display: block` by default. (https://github.com/mozdevs/cssremedy/issues/14)
2. Add `vertical-align: middle` to align replaced elements more sensibly by default. (https://github.com/jensimmons/cssremedy/issues/14#issuecomment-634934210)
   This can trigger a poorly considered lint error in some tools but is included by design.
*/

img,
svg,
video,
canvas,
audio,
iframe,
embed,
object {
  display: block;
  /* 1 */
  vertical-align: middle;
  /* 2 */
}

/*
Constrain images and videos to the parent width and preserve their intrinsic aspect ratio. (https://github.com/mozdevs/cssremedy/issues/14)
*/

img,
video {
  max-width: 100%;
  height: auto;
}

/* Make elements with the HTML hidden attribute stay hidden by default */

[hidden] {
  display: none;
}

.mx-auto {
  margin-left: auto;
  margin-right: auto;
}

.h-24 {
  height: 6rem;
}

.w-24 {
  width: 6rem;
}

.space-y-4 > :not([hidden]) ~ :not([hidden]) {
  --tw-space-y-reverse: 0;
  margin-top: calc(1rem * calc(1 - var(--tw-space-y-reverse)));
  margin-bottom: calc(1rem * var(--tw-space-y-reverse));
}

.rounded-full {
  border-radius: 9999px;
}

.rounded-xl {
  border-radius: 0.75rem;
}

.bg-slate-100 {
  --tw-bg-opacity: 1;
  background-color: rgb(241 245 249 / var(--tw-bg-opacity));
}

.p-8 {
  padding: 2rem;
}

.pt-6 {
  padding-top: 1.5rem;
}

.text-center {
  text-align: center;
}

.text-lg {
  font-size: 1.125rem;
  line-height: 1.75rem;
}

.font-medium {
  font-weight: 500;
}

.text-sky-500 {
  --tw-text-opacity: 1;
  color: rgb(14 165 233 / var(--tw-text-opacity));
}

.text-slate-700 {
  --tw-text-opacity: 1;
  color: rgb(51 65 85 / var(--tw-text-opacity));
}

.text-sky-100 {
  --tw-text-opacity: 1;
  color: rgb(224 242 254 / var(--tw-text-opacity));
}

@media (min-width: 768px) {
  .md\:flex {
    display: flex;
  }

  .md\:h-auto {
    height: auto;
  }

  .md\:w-48 {
    width: 12rem;
  }

  .md\:rounded-none {
    border-radius: 0px;
  }

  .md\:p-8 {
    padding: 2rem;
  }

  .md\:p-0 {
    padding: 0px;
  }

  .md\:text-left {
    text-align: left;
  }
}

@media (prefers-color-scheme: dark) {
  .dark\:bg-slate-800 {
    --tw-bg-opacity: 1;
    background-color: rgb(30 41 59 / var(--tw-bg-opacity));
  }

  .dark\:text-sky-400 {
    --tw-text-opacity: 1;
    color: rgb(56 189 248 / var(--tw-text-opacity));
  }

  .dark\:text-slate-500 {
    --tw-text-opacity: 1;
    color: rgb(100 116 139 / var(--tw-text-opacity));
  }
}
```
