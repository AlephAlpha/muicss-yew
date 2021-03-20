html! {
    <>
        <div>
            <Dropdown label="Dropleft" color=Color::Primary placement=Placement::Left>
                <DropdownItem link="#">
                    { "Option 1" }
                </DropdownItem>
                <DropdownItem>
                    { "Option 2" }
                </DropdownItem>
                <DropdownItem>
                    { "Option 3" }
                </DropdownItem>
                <DropdownItem>
                    { "Option 4" }
                </DropdownItem>
            </Dropdown>
        </div>
        <div>
            <Dropdown label="Dropleft (bottom aligned)" color=Color::Primary placement=Placement::Left alignment=Alignment::Bottom>
                <DropdownItem link="#">
                    { "Option 1" }
                </DropdownItem>
                <DropdownItem>
                    { "Option 2" }
                </DropdownItem>
                <DropdownItem>
                    { "Option 3" }
                </DropdownItem>
                <DropdownItem>
                    { "Option 4" }
                </DropdownItem>
            </Dropdown>
        </div>
    </>
}
