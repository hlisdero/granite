mod utils;

utils::generate_tests_for_example_program!(
    "./examples/programs/mutex/double_lock_deadlock_with_arc.rs",
    "./examples/results/mutex/double_lock_deadlock_with_arc/"
);
