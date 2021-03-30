html! {
    <>
        <div>
            <Dropdown label="Dropright" color=Color::Primary placement=Placement::Right>
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
            <Dropdown label="Dropright (bottom aligned)" color=Color::Primary placement=Placement::Right alignment=Alignment::Bottom>
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
