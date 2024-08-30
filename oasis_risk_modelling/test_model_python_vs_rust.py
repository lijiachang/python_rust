import random
import time

import matplotlib.pyplot as plt
import pandas as pd

from oasis_risk_modelling import construct_model


def python_construct_model(event_ids):
    """使用pandas重写rust的构造模型"""
    # 从csv中加载数据
    vulnerabilities = pd.read_csv("./footprint.csv")
    foot_print = pd.read_csv("./footprint.csv")
    event_ids = pd.DataFrame(event_ids)

    # 合并数据，并重命名probability列以避免冲突
    model = pd.merge(event_ids, foot_print, how="inner", on="event_id")
    model.rename(columns={"probability": "footprint_probability"}, inplace=True)

    # 最后，合并灾难脆弱性数据，然后计算总概率
    model = pd.merge(model, vulnerabilities, how="inner", on="intensity_bin_id")
    model.rename(columns={"probability": "vulnerability_probability"}, inplace=True)
    model["total_prob"] = model["footprint_probability"] * model["vulnerability_probability"]

    return model


def generate_event_ids_for_python(number_of_events):
    """
    随机事件ID生成器
    对于Python模型来说，需要一个字典列表
    """
    return [{"event_id": random.randint(1, 4)} for _ in range(number_of_events)]


def generate_event_ids_for_rust(number_of_events):
    """
    随机事件ID生成器
    对于Rust模型来说，需要一个整数列表
    """
    return [random.randint(1, 4) for _ in range(number_of_events)]


if __name__ == '__main__':
    x = []
    python_y = []
    rust_y = []

    # 测试数据
    for i in range(10, 3000, 10):
        x.append(i)

        # 测试Python的实现
        python_event_ids = generate_event_ids_for_python(number_of_events=i)
        python_start = time.time()
        python_construct_model(event_ids=python_event_ids)
        python_finish = time.time()
        python_y.append(python_finish - python_start)

        # 测试Rust的实现
        rust_event_ids = generate_event_ids_for_rust(number_of_events=i)
        rust_start = time.time()
        construct_model(event_ids=rust_event_ids)
        rust_finish = time.time()
        rust_y.append(rust_finish - rust_start)

    plt.plot(x, python_y, color='blue', label='Python')
    plt.plot(x, rust_y, color='red', label='Rust')
    plt.legend()  # 添加图例
    plt.show()
