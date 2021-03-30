html! {
    <>
        <div>
            <Dropdown label="Dropup" color=Color::Primary placement=Placement::Up>
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
            <Dropdown label="Dropup (right aligned)" color=Color::Primary placement=Placement::Up alignment=Alignment::Right>
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
