pub fn swap_values(vec: &mut Vec<u32>, rhs_idx: usize, lhs_idx: usize) {
    let tmp = vec[rhs_idx];
    vec[rhs_idx] = vec[lhs_idx];
    vec[lhs_idx] = tmp;
}