use leetcode::two_sum1;
#[test]
fn return_which_elements_summed_results_to_a_given_number() {
    assert_eq!(two_sum1::solution(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum1::solution(vec![5, 1, 7, 3], 10), vec![2, 3]);
    assert_eq!(two_sum1::solution(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum1::solution(vec![3, 3], 6), vec![0, 1]);
    assert_eq!(two_sum1::solution(vec![7, 7, 7, 7, 7, 7], 14), vec![0, 1]);
}

#[test]
#[should_panic(expected = "No solution found!")]
fn panic_when_no_solution_found() {
    two_sum1::solution(vec![1, 2], 14);
}
