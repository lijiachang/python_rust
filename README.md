# Speed Your Python with Rust: PyO3 demo
项目使用PyO3最新版本：0.22.2  
新版的PyO3除了一些用法的改变，在新版本中引入了 Bound 的概念：

    Bound 是一个新的智能指针类型，用于表示与特定 Python 解释器生命周期绑定的 Python 对象。这个改变的主要目的是提高类型安全性和明确所有权。

    导入方式的变化：

    旧版：py.import("numpy")
    新版：py.import_bound("numpy")

    新版本返回一个 Bound 对象，这更明确地表示了导入的模块与当前 Python 解释器实例的关系。

    函数签名的变化：

    旧版：pub fn run_config<'a>(config: &'a PyDict) -> PyResult<&'a PyDict>
    新版：pub fn run_config<'a>(config: &Bound<'a, PyDict>) -> PyResult<Bound<'a, PyDict>>

    这个变化使得函数签名更明确地表达了 Python 对象的生命周期和所有权。

    优势：
        更好的类型安全：Bound 可以在编译时捕获更多的潜在错误。
        明确的生命周期：使得 Rust 的借用检查器能更好地理解 Python 对象的生命周期。
        更好的所有权模型：让 Rust 代码更容易推理 Python 对象的所有权和生命周期。


## 为pip模块构建Rust接口
* 使用pip打包Rust代码
* 使用PyO3 crate创建Rust接口
* 为Rust包构建测试

## 在Rust中使用Python对象
* 将复杂的python对象传递到Rust中
* 检查和使用自定义python对象
* 在Rust中构建自定义python对象

## 在Rust中使用Python模块
* 在NumPy中构建模型
* 在Rust中使用NumPy和其他Python模块
* 在Rust中重建NumPy模型

# How to install
Choose one of the three ways:  

## pip install
```
pip install git+https://github.com/lijiachang/python_rust@master
```

## python setup.py install
```
pip3 install setuptools-rust
python setup.py build && python setup.py install
```


    python setup.py build
        这个命令编译项目中的任何 C 扩展（如您的 Rust 扩展）。
        它不会安装任何东西，只是准备项目以供安装。

    python setup.py install
        这个命令将项目安装到 Python 的 site-packages 目录中。
        它会复制所有必要的文件到安装目录，包括编译好的扩展。
        这是一个"静态"安装 - 如果您更改了源代码，需要重新运行此命令来更新安装。

    python setup.py develop
        这个命令创建一个到您的项目目录的链接，而不是复制文件。
        它仍然会编译和安装任何 C 扩展。
        主要用于开发过程中，因为它允许您修改源代码而不需要重新安装。
        对源代码的任何更改都会立即反映在已安装的包中。
        这种安装方式也被称为"可编辑"或"开发"模式。

## pip install .
```text
构建 Rust 扩展：
cargo build --release
这会在 target/release 目录下生成一个动态库文件（.so、.dll 或 .dylib）
在 Windows 上是 .dll，在 macOS 上是 .dylib，在 Linux 上是 .so

然后，运行以下命令来安装 Python 包：
pip install .
这个命令会调用 setup.py，编译 Rust 代码，并安装 Python 包。

如果安装成功，您应该能够在 Python 中导入和使用这个模块了。 
```

# python terminal verify 测试结果
hello
```text
(venv) ➜  src git:(master) ✗ python
Python 3.10.11 (v3.10.11:7d4cc5aa85, Apr  4 2023, 19:05:19) [Clang 13.0.0 (clang-1300.0.29.30)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> from pyo3_example import say_hello
>>> 
>>> say_hello()
saying hello from Rust~
saying hello from Rust~
```
fibonacci_number
```text
(venv) ➜  src git:(master) ✗ python                     
Python 3.10.11 (v3.10.11:7d4cc5aa85, Apr  4 2023, 19:05:19) [Clang 13.0.0 (clang-1300.0.29.30)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> from pyo3_example import fibonacci_number
>>> from pyo3_example import fibonacci_numbers
>>> fibonacci_number(20)
6765
>>> fibonacci_numbers([20,21,22])
[6765, 10946, 17711]
>>> 
```
通过setup.py 中定义entry_points -> console_scripts 实现为系统终端添加命令工具
```text
(venv) ➜  python_rust git:(master) ✗ fib-number                                                          
usage: fib-number [-h] --number NUMBER
fib-number: error: the following arguments are required: --number
(venv) ➜  python_rust git:(master) ✗ fib-number --number 20
your fibonacci number is:6765 
```
在python控制台中测试适配器接口
```text
(venv) ➜  src git:(master) ✗ python                                                               
Python 3.10.11 (v3.10.11:7d4cc5aa85, Apr  4 2023, 19:05:19) [Clang 13.0.0 (clang-1300.0.29.30)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> from pyo3_example import FibNumberAdapter
>>> test = FibNumberAdapter(10)
>>> test2 = FibNumberAdapter(15)
>>> test2.count
2
>>> test.count
2
>>> test2.success
True
>>> test2.result
610
>>> test_list = FibNumberAdapter([5,6,7,8,9])
>>> test_list.result
[5, 8, 13, 21, 34]
>>> test.count
3
>>> test2.count
3
>>> test_list.count
3
>>> 
```

将复杂的python对象传递到Rust中,  
config_number_command示例，读取yaml文件，转换为dict，传递给Rust处理
```text
(venv) ➜  src git:(master) ✗ pip install pyyaml
...

(venv) ➜  src git:(master) ✗ pwd                                                                  
/Users/lijiachang/code/rust/python_rust/src

(venv) ➜  src git:(master) ✗ config-fib --path ../example.yml
config data:
{'number': [4, 7, 2], 'numbers': [[12, 15, 20], [15, 19, 18]]}
the result:
{'NUMBER RESULT': [3, 13, 1],
 'NUMBERS RESULT': [[144, 610, 6765], [610, 4181, 2584]],
 'number': [4, 7, 2],
 'numbers': [[12, 15, 20], [15, 19, 18]]}
```

检查和使用自定义python对象  
```text
(venv) ➜  src git:(master) ✗ python                                                               
Python 3.10.11 (v3.10.11:7d4cc5aa85, Apr  4 2023, 19:05:19) [Clang 13.0.0 (clang-1300.0.29.30)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> from pyo3_example import ObjectInterface
>>> test = ObjectInterface([5,6,7,8], [])
>>> test.process()
>>> test.number_results
[5, 8, 13, 21]
>>> 

```

在Rust中构建自定义python对象  
可以看到Rust对象可在python系统中无缝的工作
```text
(venv) ➜  src git:(master) ✗ python
Python 3.10.11 (v3.10.11:7d4cc5aa85, Apr  4 2023, 19:05:19) [Clang 13.0.0 (clang-1300.0.29.30)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> from pyo3_example import FibProcessor
>>> test = FibProcessor([11,12,13,14], [[11,12], [13,14], [15,16]])
>>> test.numbers_results
[[89, 144], [233, 377], [610, 987]]
>>> 
>>> test.numbers_results = "write"  # 报错是因为只读
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
AttributeError: attribute 'numbers_results' of 'builtins.FibProcessor' objects is not writable
>>> 
>>> test.number = [1,2]  # 可读写
>>> test.number
[1, 2]
>>> 

```

在Rust中使用NumPy模块  
```text
(venv) ➜  src git:(master) ✗ python                                                               
Python 3.10.11 (v3.10.11:7d4cc5aa85, Apr  4 2023, 19:05:19) [Clang 13.0.0 (clang-1300.0.29.30)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> from pyo3_example import test_numpy
>>> outcome = test_numpy({})
>>> outcome["numpy result"].transpose()
array([[70, 90]])
>>> 

```

# FAQ

## ModuleNotFoundError: No module named 'pyo3_example.pyo3_example'
不要在项目根目录（setup.py、Cargo.toml所在目录）执行python终端，因为[pyo3_example](pyo3_example)目录会冲突