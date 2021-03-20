html! {
    <>
        <div>
            <Dropdown label="Dropdown" color=Color::Primary>
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
            <Dropdown label="Dropdown (right aligned)" color=Color::Primary alignment=Alignment::Right>
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
