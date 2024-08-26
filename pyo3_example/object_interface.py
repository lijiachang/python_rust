from typing import List, Optional

from .pyo3_example import object_interface


class ObjectInterface:
    def __init__(self, number: List[int], numbers: List[List[int]]):
        self.number = number
        self.numbers = numbers

        self.number_results: Optional[List[int]] = None
        self.numbers_results: Optional[List[List[int]]] = None

    def process(self):
        object_interface(self)  # 通过self传递给rust的函数
