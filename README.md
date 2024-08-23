# Speed Your Python with Rust: PyO3 demo
* 使用pip打包Rust代码
* 使用PyO3 crate创建Rust接口

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
这会在 target/release 目录下生成一个动态库文件（.so、.dll 或 .dylib）。

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

# FAQ

## ModuleNotFoundError: No module named 'pyo3_example.pyo3_example'
不要在项目根目录（setup.py、Cargo.toml所在目录）执行python终端，因为[pyo3_example](pyo3_example)目录会冲突