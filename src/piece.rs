
struct Coords {
    x: u32,
    y: u32,
}


struct Piece {
    render_value: char,
    image_path: String,
    movement_fn: fn(&self, &Board, Option<&Fn>) -> Vec<Coords>,
    player: Player,

}

impl Piece {

    fn get_render(&self) -> char {
       self.render_value 
    }
}


fn king_move(&self, board: &Board) -> Vec<Coords> {

}