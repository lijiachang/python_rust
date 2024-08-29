use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyDict;

mod fib_calcs;
mod interface;
mod class_module;
mod numpy_model;
mod footprint;
mod vulnerabilities;
// use fib_calcs::fib_number::__pyo3_get_function_fibonacci_number; // 旧版写法
// use fib_calcs::fib_numbers::__pyo3_get_function_fibonacci_numbers;// 旧版写法
// 使用__pyo3_get_function_前缀，使我们能够保留应用于函数的宏。如果直接导入函数，则无法将他们添加到模块中，这会导致在安装包时出现编译错误。 但是新版pyo3好像不需要了？

use footprint::merge_event_ids_with_footprint;
use vulnerabilities::merge_vulnerabilities_with_footprint;

#[pyfunction]
fn say_hello() {
    println!("saying hello from Rust~");
    println!("saying hello from Rust~")
}

#[pyfunction]
fn test_numpy<'a>(result_dict: &'a Bound<'a, PyDict>) -> PyResult<&'a Bound<'a, PyDict>> {
    Python::with_gil(|py| {
        let locals = PyDict::new_bound(py);
        locals.set_item("np", py.import_bound("numpy").unwrap())?;

        let code = "np.array([[3, 2], [1, 4]])";
        let weights_matrix = py.eval_bound(code, None, Some(&locals)).unwrap();
        locals.set_item("weights_matrix", weights_matrix)?;

        let new_code = "np.array([[10], [20]])";
        let input_matrix = py.eval_bound(new_code, None, Some(&locals)).unwrap();
        locals.set_item("input_matrix", input_matrix)?;

        let calc_code = "np.dot(weights_matrix, input_matrix)";
        let numpy_result = py.eval_bound(calc_code, None, Some(&locals)).unwrap();
        result_dict.set_item("numpy result", numpy_result)?;

        return Ok(result_dict);
    })
}


#[pyfunction]
fn get_model(event_ids: Vec<i32>, base_path: String, py: Python) -> Vec<Bound<PyDict>> {
    let footprints = merge_event_ids_with_footprint(event_ids, base_path.clone());
    let model = merge_vulnerabilities_with_footprint(footprints, base_path);

    let mut buffer = Vec::new();
    for i in model {
        let placeholder = PyDict::new_bound(py);
        placeholder.set_item("vulnerability_id", i.vulnerability_id).unwrap();
        placeholder.set_item("intensity_bin_id", i.intensity_bin_id).unwrap();
        placeholder.set_item("damage_bin_id", i.damage_bin_id).unwrap();
        placeholder.set_item("damage_probability", i.damage_probability).unwrap();
        placeholder.set_item("event_id", i.event_id).unwrap();
        placeholder.set_item("areaperil_id", i.areaperil_id).unwrap();
        placeholder.set_item("footprint_probability", i.footprint_probability).unwrap();
        placeholder.set_item("total_probability", i.total_probability).unwrap();
        buffer.push(placeholder);
    }
    buffer
}


#[pymodule]
fn pyo3_example(module: &Bound<'_, PyModule>) -> PyResult<()> {
    // module.add_wrapped(wrap_pyfunction!(say_hello));  // 旧版pyo3写法
    let _ = module.add_function(wrap_pyfunction!(say_hello, module)?); // 将函数添加到模块中

    let _ = module.add_function(wrap_pyfunction!(fib_calcs::fib_number::fibonacci_number, module)?);
    let _ = module.add_function(wrap_pyfunction!(fib_calcs::fib_numbers::fibonacci_numbers, module)?);

    let _ = module.add_function(wrap_pyfunction!(interface::config::run_config, module)?);
    let _ = module.add_function(wrap_pyfunction!(interface::object::object_interface, module)?);
    let _ = module.add_class::<class_module::fib_processor::FibProcessor>();  // 将类添加到模块中

    let _ = module.add_function(wrap_pyfunction!(test_numpy, module)?);

    let _ = module.add_function(wrap_pyfunction!(numpy_model::calculate_times, module)?);
    let _ = module.add_function(wrap_pyfunction!(numpy_model::calculate_parameters, module)?);

    let _ = module.add_function(wrap_pyfunction!(get_model, module)?);

    Ok(())
}

// #[pymodule]
// fn oasis_risk_modelling(module: &Bound<'_, PyModule>) -> PyResult<()> {
//     let _ = module.add_function(wrap_pyfunction!(get_model, module)?);
//     Ok(())
// }