use sdl_isometric::ascii::GameState;

fn main() {
    let mut state = GameState::new();

    state.add_entity(Some((1, 1)), Some(sdl_isometric::ascii::Tile::Wall));
    state.add_entity(Some((2, 3)), Some(sdl_isometric::ascii::Tile::Ascii('c')));
    dbg!(state);
}
