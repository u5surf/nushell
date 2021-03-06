use nu_test_support::{nu, pipeline};

#[test]
fn median_numbers_with_even_rows() {
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
             echo [10 6 19 21 4]
             | math median
             | echo $it
         "#
    ));

    assert_eq!(actual.out, "10")
}

#[test]
fn median_numbers_with_odd_rows() {
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
             echo [3 8 9 12 12 15]
             | math median
             | echo $it
         "#
    ));

    assert_eq!(actual.out, "10.5")
}
