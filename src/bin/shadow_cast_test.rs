use sdl_isometric::physics::shadow_casting::*;

fn main() {
    let mut light_map: LightMap = LightMap::new(10, 5);

    light_map.calculate_pov(3, (3, 3));

    for y in 0..light_map.height {
        for x in 0..light_map.width {
            let idx = (y * light_map.width + x) as usize;

            print!(
                "{} ",
                match light_map.visible.get(idx).unwrap() {
                    true => "O",
                    _ => ".",
                }
            )
        }
        println!("");
    }
}
