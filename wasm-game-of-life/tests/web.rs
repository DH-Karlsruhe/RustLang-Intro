//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

// test by running: `wasm-pack test --firefox --headless`

extern crate wasm_game_of_life;
use wasm_game_of_life::universe::Universe;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    use std::option::Option::None;

    let mut universe = Universe::new(None, None);
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new(None, None);
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(2,1), (2,3), (3,2), (3,3), (4,2)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = input_spaceship();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_spaceship();

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}

// Differs from default rust test modules,
// because of the headless-browser implementation.
// Usually test-modules are written like..
/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick() {
        // ...
    }
} */