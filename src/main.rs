mod sorting;
use crate::sorting::bubble_sort::test_bubble_sort;

fn main() {
    let mut vec = vec![3, 9, 12, 8, 0, 78, 539238472, 2];
    let mut vec2 = vec![1];
    let mut vec3: Vec<u32> = vec![];
    test_bubble_sort(&mut vec);
    test_bubble_sort(&mut vec2);
    test_bubble_sort(&mut vec3);
}
