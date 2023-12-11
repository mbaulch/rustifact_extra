use rustifact::ToTokenStream;
use rustifact_extra::BareJaggedArrayBuilder;

fn main() {
    let mut num_array = BareJaggedArrayBuilder::new();
    num_array.push(vec![1, 2, 3]);
    num_array.push(vec![4]);
    num_array.push(vec![5, 6]);
    let row1 = num_array.get_precalc("NUM_ARRAY", 0);
    let row2 = num_array.get_precalc("NUM_ARRAY", 1);
    let row3 = num_array.get_precalc("NUM_ARRAY", 2);
    rustifact::write_const!(NUM_ARRAY_ELEMS_LEN, usize, num_array.elems_len());
    rustifact::write_static!(NUM_ARRAY_ROW1, &[i32], &row1);
    rustifact::write_static!(NUM_ARRAY_ROW2, &[i32], &row2);
    rustifact::write_static!(NUM_ARRAY_ROW3, &[i32], &row3);
    rustifact::write_static!(NUM_ARRAY, BareJaggedArray<i32, NUM_ARRAY_ELEMS_LEN>, &num_array);
}
