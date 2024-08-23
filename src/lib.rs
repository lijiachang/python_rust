use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn say_hello() {
    println!("saying hello from Rust~")
}


#[pymodule]
fn python_rust(module: &Bound<'_, PyModule>) -> PyResult<()> {
    // module.add_wrapped(wrap_pyfunction!(say_hello));  // 旧版pyo3写法
    module.add_function(wrap_pyfunction!(say_hello, module)?)
}