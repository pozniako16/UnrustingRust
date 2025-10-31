use crate::sorting::utils::swap_values;

pub fn insertion_sort(vec: &mut Vec<u32>) -> u32 {
    if vec.len() <= 1 {
        return 0
    }
    let mut iterations = 0;
    for i in 0..vec.len() {
        let mut mini_idx = i;
        for j in i+1..vec.len() {
            if j < mini_idx {
                mini_idx = j;
            }
            iterations += 1;
        }
        if i != mini_idx {
            swap_values(vec, i, mini_idx);
        }
    }
    iterations
}
