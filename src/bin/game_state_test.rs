use sdl_isometric::ascii::game_state::GameState;
use sdl_isometric::ascii::Tile;

fn main() {
    let mut state = GameState::new();

    state.add_entity(Some((1, 1)), Some(Tile::Wall));
    state.add_entity(Some((2, 3)), Some(Tile::Ascii('c')));
    state.add_entity(None, Some(Tile::Ascii('d')));
    state.add_entity(None, Some(Tile::Player));

    let entities = state.entity_coord_and_tile();

    dbg!(entities);
}
