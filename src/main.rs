mod mazegen;

use mazegen::carve_iterative::carve_iterative;

fn main() {
    let rows = 2;
    let cols = 2;
    let verbose = true;
    carve_iterative(rows, cols, verbose);
}
