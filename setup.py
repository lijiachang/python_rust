from setuptools import dist

dist.Distribution().fetch_build_eggs(['setuptools_rust'])

from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="python-rust",
    version="0.1",

    rust_extensions=[RustExtension(".python_rust.python_rust", path="Cargo.toml", binding=Binding.PyO3)],
    # 第一个python_rust是rust项目的名字（Cargo.toml中定义），第二个python_rust是在lib.rs中的#[pymodule]定义的

    packages=["python_rust"],
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
    ],
    zip_safe=False,  # rust extension is not zip safe 这也是C语言的标准
)
