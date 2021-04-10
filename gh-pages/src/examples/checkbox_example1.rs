html! {
    <>
        <div>
            <Checkbox value="opt1" >
                { "Option one" }
            </Checkbox>
        </div>
        <div>
            <Checkbox value="opt2" checked=true >
                { "Option two" }
            </Checkbox>
        </div>
        <div>
            <Checkbox value="opt3" checked=true disabled=true >
                { "Option three is disabled" }
            </Checkbox>
        </div>
    </>
}
