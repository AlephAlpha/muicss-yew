# muicss-yew

MUICSS-yew is a component library for [Yew](https://yew.rs) framework based on the lightweight [MUI](https://www.muicss.com/) CSS framework.

:construction: This project is still work in progress. Many components are not yet supported. If those components are needed, you may use MUI-CSS directly (i.e., [with CSS and JS](https://www.muicss.com/docs/v1/css-js/boilerplate-html)) with Yew.

For other Yew component libraries, please see [awesome-yew](https://github.com/jetli/awesome-yew#component-libraries).

## Usage

Add these two lines in your static html file:

```html
<link rel="stylesheet" href="https://cdn.muicss.com/mui-0.10.3/css/mui.min.css" />
<script src="https://cdn.muicss.com/mui-0.10.3/js/mui.min.js"></script>
```

Add muicss-yew to your `Cargo.toml`:

```toml
[dependencies]
muicss-yew = { git = "https://github.com/AlephAlpha/muicss-yew" }
```

For details, please see the following examples.

## Supported components and examples

- [x] [Appbar](https://alephalpha.github.io/muicss-yew/#appbar)
- [x] [Buttons](https://alephalpha.github.io/muicss-yew/#buttons)
- [x] [Caret](https://alephalpha.github.io/muicss-yew/#caret)
- [x] [Checkbox](https://alephalpha.github.io/muicss-yew/#checkbox)
- [x] [Container](https://alephalpha.github.io/muicss-yew/#container)
- [x] [Divider](https://alephalpha.github.io/muicss-yew/#divider)
- [x] [Dropdowns](https://alephalpha.github.io/muicss-yew/#dropdowns)
- [x] [Grid](https://alephalpha.github.io/muicss-yew/#grid)
- [x] [Input](https://alephalpha.github.io/muicss-yew/#input)
- [x] [Panels](https://alephalpha.github.io/muicss-yew/#panels)
- [x] [Textarea](https://alephalpha.github.io/muicss-yew/#textarea)