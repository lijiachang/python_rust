import os
# from .oasis_risk_modelling import get_model
from pyo3_example import get_model


def construct_model(event_ids):
    dir_path = os.path.dirname(os.path.realpath(__file__))

    # print(dir_path)
    # debug 打印下路径:/Users/lijiachang/code/rust/python_rust/venv/lib/python3.10/site-packages/oasis_risk_modelling

    return get_model(event_ids, str(dir_path))
