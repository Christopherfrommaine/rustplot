#[allow(unused_imports)] // inputs are used, but doesn't detect it?
use crate::helper::arrays::*;

#[test]
fn pad_table_test_1() {
    let tab = vec![vec![0, 1, 2], vec![2, 3, 4]];
    let left = pad_table(&tab, 9, ((1, 2), (3, 4)));
    let right = 
        vec![
            vec![9, 9, 9, 9, 9, 9],
            vec![9, 9, 9, 9, 9, 9],
            vec![9, 9, 9, 9, 9, 9],
            vec![9, 0, 1, 2, 9, 9],
            vec![9, 2, 3, 4, 9, 9],
            vec![9, 9, 9, 9, 9, 9],
            vec![9, 9, 9, 9, 9, 9],
            vec![9, 9, 9, 9, 9, 9],
            vec![9, 9, 9, 9, 9, 9],
        ];

    println!("{:?}", right);

    assert_eq!(left, right)
}
