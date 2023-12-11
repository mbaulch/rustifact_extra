use rustifact::ToTokenStream;
use rustifact_extra::JaggedArrayBuilder;

fn main() {
    let mut num_array = JaggedArrayBuilder::new();
    num_array.push(vec![1, 2, 3]);
    num_array.push(vec![4]);
    num_array.push(vec![5, 6]);
    rustifact::write_const!(NUM_ARRAY_LEN, usize, num_array.len());
    rustifact::write_const!(NUM_ARRAY_ELEMS_LEN, usize, num_array.elems_len());
    rustifact::write_static!(NUM_ARRAY, JaggedArray<i32, NUM_ARRAY_LEN, NUM_ARRAY_ELEMS_LEN>, &num_array);
}
