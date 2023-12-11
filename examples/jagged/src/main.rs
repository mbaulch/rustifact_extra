rustifact::use_symbols!(NUM_ARRAY, NUM_ARRAY_LEN, NUM_ARRAY_ELEMS_LEN);
use rustifact_extra::JaggedArray;

fn main() {
    assert_eq!(NUM_ARRAY[0], [1, 2, 3]);
    assert_eq!(NUM_ARRAY[1], [4]);
    assert_eq!(NUM_ARRAY[2], [5, 6]);
}
