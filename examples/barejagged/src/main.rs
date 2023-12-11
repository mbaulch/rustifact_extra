rustifact::use_symbols!(
    NUM_ARRAY,
    NUM_ARRAY_ELEMS_LEN,
    NUM_ARRAY_ROW1,
    NUM_ARRAY_ROW2,
    NUM_ARRAY_ROW3
);
use rustifact_extra::BareJaggedArray;

fn main() {
    assert_eq!(NUM_ARRAY_ROW1, [1, 2, 3]);
    assert_eq!(NUM_ARRAY_ROW2, [4]);
    assert_eq!(NUM_ARRAY_ROW3, [5, 6]);
}
