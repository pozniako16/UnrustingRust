use crate::sorting::utils::swap_values;

fn bubble_sort(vec: &mut Vec<u32>) -> u32 {
    if vec.len() <= 1 {
        return 0
    }
    let mut iterations = 0;
    for index in 0..vec.len(){
        let mut swapped = false;
        for pivot in index..vec.len() {
            if vec[index] > vec[pivot] {
                swap_values(vec, index, pivot);
                swapped=true
            }
            iterations += 1;
        }
        if !swapped {
            return iterations;
        }
    }
    iterations
}

pub fn test_bubble_sort(vec: &mut Vec<u32>) {
    println!("______________________________");
    println!("Before: {:?}", vec);
    let it = bubble_sort(vec);
    println!("After: {:?}", vec);
    println!("Iterations: {}", it);
    println!("______________________________");
}
