struct Board {
    current_state: Vec<Square>,
    window: OurGuiWindowPntr
}

impl Board {
    fn display(self) {
        for state in self.current_state {
            gui.draw(state.square, x=state.square.x, y=state.square.y)
            // Draw(filename, xcoord, ycoord)
        }
    }
}