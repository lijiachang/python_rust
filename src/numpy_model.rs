use pyo3::prelude::*;
use pyo3::types::PyDict;

/// 权重矩阵函数
fn get_weight_matrix(py: &Python, locals: &Bound<PyDict>) -> () {
    let code: &str = "np.array([[3, 2], [1, 4]])";
    let weight_matrix = py.eval_bound(code, None, Some(locals)).unwrap();
    locals.set_item("weights_matrix", weight_matrix).unwrap()
}

/// 创建逆矩阵函数
fn invert_get_weight_matrix(py: &Python, locals: &Bound<PyDict>) -> () {
    let code: &str = "np.linalg.inv(weights_matrix)";
    let inverted_weights_matrix = py.eval_bound(code, None, Some(locals)).unwrap();
    locals.set_item("inverted_weight_matrix", inverted_weights_matrix).unwrap()
}

/// 输入向量函数
fn get_input_vector(py: &Python, locals: &Bound<PyDict>, first: i32, second: i32) -> () {
    let code: String = format!("np.array([[{}], [{}]])", first, second);
    let input_vector = py.eval_bound(&code, None, Some(locals)).unwrap();
    locals.set_item("input_vector", input_vector).unwrap()
}

/// 计算np.dot
fn get_times<'a>(py: &'a Python, locals: &'a Bound<PyDict>) -> Bound<'a, PyAny> {
    let code: &str = "np.dot(weights_matrix, input_vector)";
    let times = py.eval_bound(code, None, Some(locals)).unwrap();
    times
}

/// 计算np.dot
fn get_parameters<'a>(py: &'a Python, locals: &'a Bound<PyDict>) -> Bound<'a, PyAny> {
    let code: &str = "np.dot(inverted_weight_matrix, input_vector)";
    let parameters = py.eval_bound(code, None, Some(locals)).unwrap();
    parameters
}

///
#[pyfunction]
pub fn calculate_times<'a>(result_dict: &'a Bound<'a, PyDict>, distance: i32, traffic_grade: i32) -> PyResult<&'a Bound<'a, PyDict>> {
    Python::with_gil(|py| {
        let locals = PyDict::new_bound(py);
        locals.set_item("np", py.import_bound("numpy").unwrap())?;

        get_weight_matrix(&py, &locals);
        get_input_vector(&py, &locals, distance, traffic_grade);
        result_dict.set_item("times", get_times(&py, &locals))?;
        Ok(result_dict)
    })
}

#[pyfunction]
pub fn calculate_parameters<'a>(result_dict: &'a Bound<'a, PyDict>, car_time: i32, trunk_time:i32) -> PyResult<&'a Bound<'a, PyDict>> {
    Python::with_gil(|py|{
        let locals = PyDict::new_bound(py);
        locals.set_item("np", py.import_bound("numpy").unwrap())?;

        get_weight_matrix(&py, &locals);
        invert_get_weight_matrix(&py, &locals);
        get_input_vector(&py,&locals, car_time, trunk_time);

        result_dict.set_item("parameters", get_parameters(&py, &locals))?;

        Ok(result_dict)

    })
}