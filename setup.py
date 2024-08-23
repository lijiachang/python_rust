from setuptools import dist

dist.Distribution().fetch_build_eggs(['setuptools_rust'])

from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="pyo3-example",
    version="0.1",

    rust_extensions=[RustExtension(".pyo3_example.pyo3_example", path="Cargo.toml", binding=Binding.PyO3)],
    # 第一个pyo3_example是下面packages定义的名字（也是项目中的pyo3_example目录），
    # 第二个pyo3_example是在 lib.rs 中的#[pymodule]定义的fn pyo3_example(module: &Bound<'_, PyModule>)

    packages=["pyo3_example"],
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
