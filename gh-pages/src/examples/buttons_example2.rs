html! {
    <>
        <div>
            <Button variant=Variant::Flat> { "Button" } </Button>
            <Button variant=Variant::Flat color=Color::Primary> { "Button" } </Button>
            <Button variant=Variant::Flat color=Color::Danger> { "Button" } </Button>
            <Button variant=Variant::Flat color=Color::Accent> { "Button" } </Button>
            <Button variant=Variant::Flat color=Color::Dark> { "Button" } </Button>
        </div>
        <div>
            <Button variant=Variant::Flat disabled=true> { "Button" } </Button>
            <Button variant=Variant::Flat color=Color::Primary disabled=true> { "Button" } </Button>
            <Button variant=Variant::Flat color=Color::Danger disabled=true> { "Button" } </Button>
            <Button variant=Variant::Flat color=Color::Accent disabled=true> { "Button" } </Button>
            <Button variant=Variant::Flat color=Color::Dark disabled=true> { "Button" } </Button>
        </div>
    </>
}
