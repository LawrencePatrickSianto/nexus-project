use nexus_rt::write_log;

pub fn run_addition(a: u32, b: u32, expected_result: u32) {
    let result = a + b;
    assert_eq!(result, expected_result);

    write_log("Addition ran successfully!\n");
}