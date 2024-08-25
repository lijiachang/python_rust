use pyo3::prelude::{pyfunction, PyResult};
use pyo3::types::{PyDict, PyList};
use pyo3::exceptions::PyTypeError;
use pyo3::IntoPy;
use crate::fib_calcs::fib_number::fibonacci_number;
use crate::fib_calcs::fib_numbers::fibonacci_numbers;

/// 批量计算fibonacci结果列表
fn process_numbers(input_numbers: Vec<Vec<i32>>) -> Vec<Vec<u64>> {
    let mut buffer: Vec<Vec<u64>> = Vec::new();

    for i in input_numbers {
        buffer.push(fibonacci_numbers(i));
    }

    buffer
}

#[pyfunction]
pub fn run_config<'a>(config: &'a PyDict) -> PyResult<&'a PyDict> {
    // 处理number键
    match config.get_item("number") {
        Some(data) => {
            match data.downcast::<PyList>() { //将数据向下转换为PyList结构体
                Ok(raw_data) => {
                    // 运行斐波那契数列计算函数
                    let processed_results: Vec<i32> = raw_data.extract::<Vec<i32>>().unwrap(); // todo extract
                    let fib_numbers: Vec<u64> = processed_results.iter().map(|x| fibonacci_number(*x)).collect();
                    config.set_item("NUMBER RESULT", fib_numbers)
                }
                Err(_) => Err(PyTypeError::new_err("parameter number is not a list of int"))?
            }
        }
        None => println!("parameter [number] is not in config")
    }

    // 处理numbers键
    match config.get_item("numbers") {
        Some(data) => {
            match data.downcast::<PyList>() {
                Ok(raw_data) => {
                    let processed_results_two: Vec<Vec<i32>> = raw_data.extract::<Vec<Vec<i32>>>().unwrap();
                    config.set_item("NUMBERS RESULT", process_numbers(processed_results_two))
                }
                Err(_) => Err(PyTypeError::new_err("parameter numbers is not a list of of lists of int"))?
            }
        }
        None => println!("parameter [numbers] is not in config")
    }

    Ok(config)
}