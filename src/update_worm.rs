use crate::Cell;
use crate::CellApi;
use crate::Species;
use crate::EMPTY_CELL;

pub fn update_worm(cell: Cell, mut api: CellApi) {
    let down_x = api.rand_dir_2();
    let below = api.get(0, 1);
    if below.species == Species::Empty || below.species == Species::Water {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(down_x, 1).species == Species::Empty
        || api.get(down_x, 1).species == Species::Water
    {
        api.set(0, 0, EMPTY_CELL);
        api.set(down_x, 1, cell);
    } else if below.species == Species::Wall
        || below.species == Species::Plant
        || below.species == Species::Sand
        || below.species == Species::Seed
    {
        api.set(0, 0, EMPTY_CELL);
        api.set(down_x, 1, cell);
    } else if below.species == Species::Stone && api.get(-1, 0).species == Species::Empty
        || api.get(-1, 0).species == Species::Sand
        || api.get(-1, 0).species == Species::Plant
    {
        api.set(0, 0, EMPTY_CELL);
        api.set(-1, 0, cell);
    } else if below.species == Species::Stone && api.get(1, 0).species == Species::Empty
        || api.get(1, 0).species == Species::Sand
        || api.get(1, 0).species == Species::Plant
    {
        api.set(0, 0, EMPTY_CELL);
        api.set(1, 0, cell);
    } else if below.species == Species::Stone
        && api.get(1, 0).species == Species::Stone
        && api.get(-1, 0).species == Species::Stone
    {
        api.set(0, 0, EMPTY_CELL);
        api.set(-1, 0, cell);
    } else {
        api.set(0, 0, cell);
    }
}
