html! {
    <>
        <div>
            <Button variant=Variant::Fab> { "+" } </Button>
            <Button variant=Variant::Fab color=Color::Primary> { "+" } </Button>
            <Button variant=Variant::Fab color=Color::Danger> { "+" } </Button>
            <Button variant=Variant::Fab color=Color::Accent> { "+" } </Button>
            <Button variant=Variant::Fab color=Color::Dark> { "+" } </Button>
        </div>
        <div>
            <Button variant=Variant::Fab disabled=true> { "+" } </Button>
            <Button variant=Variant::Fab color=Color::Primary disabled=true> { "+" } </Button>
            <Button variant=Variant::Fab color=Color::Danger disabled=true> { "+" } </Button>
            <Button variant=Variant::Fab color=Color::Accent disabled=true> { "+" } </Button>
            <Button variant=Variant::Fab color=Color::Dark disabled=true> { "+" } </Button>
        </div>
    </>
}
