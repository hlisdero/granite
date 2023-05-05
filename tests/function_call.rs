mod utils;

mod diverging {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/function_call/diverging.rs",
        "./examples/results/function_call/diverging/"
    );
}

mod empty_function {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/function_call/empty_function.rs",
        "./examples/results/function_call/empty_function/"
    );
}

mod in_a_loop {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/function_call/in_a_loop.rs",
        "./examples/results/function_call/in_a_loop/"
    );
}