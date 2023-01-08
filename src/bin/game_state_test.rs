use sdl_isometric::ascii::GameState;

fn main() {
    let mut state = GameState::new();

    state.add_entity(Some((1, 1)), Some(sdl_isometric::ascii::Tile::Wall));
    state.add_entity(Some((2, 3)), Some(sdl_isometric::ascii::Tile::Ascii('c')));
    state.add_entity(None, Some(sdl_isometric::ascii::Tile::Ascii('d')));
    state.add_entity(None, Some(sdl_isometric::ascii::Tile::Player));

    let entities = state.entity_coord_and_tile();

    dbg!(entities);
}
