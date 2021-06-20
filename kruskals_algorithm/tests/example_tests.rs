use kruskals_algorithm::run;
use test_case::test_case;

#[test_case(1 => 280)]
#[test_case(2 => 0)]
#[test_case(3 => 600)]
#[test_case(4 => 9500)]
#[test_case(5 => 2500)]
#[test_case(6 => 2700)]
#[test_case(7 => 1500)]
fn integration_test(dataset_number: u32) -> i32 {
    run(format!("da{}.txt", dataset_number)).unwrap()
}