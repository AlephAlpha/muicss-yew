html! {
    <>
        <div>
            <Button> { "Button" } </Button>
            <Button color=Color::Primary> { "Button" } </Button>
            <Button color=Color::Danger> { "Button" } </Button>
            <Button color=Color::Accent> { "Button" } </Button>
            <Button color=Color::Dark> { "Button" } </Button>
        </div>
        <div>
            <Button disabled=true> { "Button" } </Button>
            <Button color=Color::Primary disabled=true> { "Button" } </Button>
            <Button color=Color::Danger disabled=true> { "Button" } </Button>
            <Button color=Color::Accent disabled=true> { "Button" } </Button>
            <Button color=Color::Dark disabled=true> { "Button" } </Button>
        </div>
    </>
}
