html! {
    <>
        <div>
            <Checkbox label="Option one" value="opt1" />
        </div>
        <div>
            <Checkbox label="Option two" value="opt2" checked=true />
        </div>
        <div>
            <Checkbox label="Option three is disabled"
                value="opt3"
                checked=true
                disabled=true />
        </div>
    </>
}
