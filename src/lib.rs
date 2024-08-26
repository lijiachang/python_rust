use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod fib_calcs;
mod interface;
// use fib_calcs::fib_number::__pyo3_get_function_fibonacci_number; // 旧版写法
// use fib_calcs::fib_numbers::__pyo3_get_function_fibonacci_numbers;// 旧版写法
// 使用__pyo3_get_function_前缀，使我们能够保留应用于函数的宏。如果直接导入函数，则无法将他们添加到模块中，这会导致在安装包时出现编译错误。 但是新版pyo3好像不需要了？


#[pyfunction]
fn say_hello() {
    println!("saying hello from Rust~");
    println!("saying hello from Rust~")
}


#[pymodule]
fn pyo3_example(module: &Bound<'_, PyModule>) -> PyResult<()> {
    // module.add_wrapped(wrap_pyfunction!(say_hello));  // 旧版pyo3写法
    let _ = module.add_function(wrap_pyfunction!(say_hello, module)?);

    let _ = module.add_function(wrap_pyfunction!(fib_calcs::fib_number::fibonacci_number, module)?);
    let _ = module.add_function(wrap_pyfunction!(fib_calcs::fib_numbers::fibonacci_numbers, module)?);

    let _ = module.add_function(wrap_pyfunction!(interface::config::run_config, module)?);
    let _ = module.add_function(wrap_pyfunction!(interface::object::object_interface, module)?);

    Ok(())
}