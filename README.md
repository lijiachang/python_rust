# Speed Your Python with Rust: PyO3 demo
* 使用pip打包Rust代码

# How to install

## pip install
```
pip install git+https://github.com/lijiachang/python_rust@master
```

## python setup.py install
```
pip3 install setuptools-rust
python setup.py build && python setup.py install && python setup.py develop
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
