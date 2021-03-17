html! {
    <>
        <div>
            <Button variant=Variant::Raised> { "Button" } </Button>
            <Button variant=Variant::Raised color=Color::Primary> { "Button" } </Button>
            <Button variant=Variant::Raised color=Color::Danger> { "Button" } </Button>
            <Button variant=Variant::Raised color=Color::Accent> { "Button" } </Button>
            <Button variant=Variant::Raised color=Color::Dark> { "Button" } </Button>
        </div>
        <div>
            <Button variant=Variant::Raised disabled=true> { "Button" } </Button>
            <Button variant=Variant::Raised color=Color::Primary disabled=true> { "Button" } </Button>
            <Button variant=Variant::Raised color=Color::Danger disabled=true> { "Button" } </Button>
            <Button variant=Variant::Raised color=Color::Accent disabled=true> { "Button" } </Button>
            <Button variant=Variant::Raised color=Color::Dark disabled=true> { "Button" } </Button>
        </div>
    </>
}
