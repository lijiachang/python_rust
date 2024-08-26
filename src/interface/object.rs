use pyo3::prelude::{pyfunction, PyResult, Bound, Python, PyAnyMethods}; // 这里导入的Python是一个结构体
use pyo3::types::{PyAny, PyDict};
use pyo3::exceptions::PyLookupError;

use super::config::run_config;

#[pyfunction]
pub fn object_interface<'a>(input_object: &Bound<'a, PyAny>) -> PyResult<Bound<'a, PyAny>> {
    Python::with_gil(|py| {
        let mut config_dict: Bound<'_, PyDict> = PyDict::new_bound(py);
        extract_data(input_object, "number", &mut config_dict)?;
        extract_data(input_object, "numbers", &mut config_dict)?;

        let output_dict: Bound<PyDict> = run_config(&config_dict)?; // 交给旧函数run_config处理

        input_object.setattr("number_results", output_dict.get_item("NUMBER RESULT")?)?;
        input_object.setattr("numbers_results", output_dict.get_item("NUMBERS RESULT")?)?;

        Ok(input_object.clone())
    })
}

/// 把python对象的属性提取出来，放到dict中。 也就是把一个对象转换为dict
fn extract_data<'a>(input_object: &Bound<'a, PyAny>, attribute: &'a str, config_dict: &mut Bound<'a, PyDict>) -> PyResult<()> {
    match input_object.getattr(attribute) {
        Ok(data) => {
            config_dict.set_item(attribute, data)?;
            Ok(())
        }
        Err(_) => Err(PyLookupError::new_err(format!("attribute {} is missing", attribute)))?
    }
}