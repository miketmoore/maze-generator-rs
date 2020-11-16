mod mazegen;

use mazegen::carve_iterative::carve_iterative;

fn main() {
    let rows = 100;
    let cols = 100;
    let verbose = false;
    carve_iterative(rows, cols, verbose);
}
