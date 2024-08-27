import numpy as np


class MatrixModel:

    @property
    def weights_matrix(self) -> np.array:
        return np.array([
            [3, 2],
            [1, 4],
        ])

    def calculate_times(self, distance: int, traffic_grade: int) -> dict:
        """
        该函数将调用接受MatrixModel类的路程距离和交通流量等级的函数
        :param distance:
        :param traffic_grade:
        :return:
        """
        # 输入矩阵
        inputs = np.array([
            [distance],
            [traffic_grade]
        ])
        # 使用dot函数将它乘以权重矩阵
        result = np.dot(self.weights_matrix, inputs)
        return {
            "car time": result[0][0],
            "truck time": result[1][0],
        }

    def calculate_parameters(self, car_time: int, truck_time: int) -> dict:
        inputs = np.array([
            [car_time],
            [truck_time]
        ])
        result = np.dot(np.linalg.inv(self.weights_matrix), inputs)
        return {
            "distance": result[0][0],
            "traffic grade": result[1][0]
        }


test = MatrixModel()
times = test.calculate_times(distance=10, traffic_grade=3)
print(f"here are the times: {times}")
# here are the times: {'car time': np.int64(36), 'truck time': np.int64(22)}


parameters = test.calculate_parameters(car_time=times["car time"], truck_time=times["truck time"])
print(f"here are the parameters: {parameters}")
# here are the parameters: {'distance': np.float64(9.999999999999998), 'traffic grade': np.float64(3.0)}

# 两个矩阵交换前后顺序：使用transpose函数可以翻转矩阵, 即交换矩阵的行和列
t = np.array([
    [3, 2],
    [1, 4]
])
print(t.transpose())
"""
[[3 1]
 [2 4]]
"""
t2 = np.array([[3], [1]])
print(t2.transpose())
"""
[[3 1]]
"""


# 知识补充
"""
1. np.dot() 函数用于计算两个数组的点积。在这种情况下，它用于矩阵乘法。

假设 self.weights_matrix 是：

[[3, 2],
 [1, 4]]

而 inputs 是：

[[distance],
 [traffic_grade]]

那么 np.dot(self.weights_matrix, inputs) 的结果将是：

[[3*distance + 2*traffic_grade],
 [1*distance + 4*traffic_grade]]
这实际上是在计算一个线性组合。



2. np.linalg.inv() 函数用于计算矩阵的逆。对于一个2x2的矩阵：

[[a, b],
 [c, d]]

它的逆（如果存在）是：

1/(ad-bc) * [[ d, -b],
             [-c,  a]]

在这个例子中，self.weights_matrix 的逆是：

1/10 * [[ 4, -2],
        [-1,  3]]

    np.dot(np.linalg.inv(self.weights_matrix), inputs)

这个操作是在"撤销"之前的矩阵乘法。它使用权重矩阵的逆来从结果（汽车时间和卡车时间）推导出原始输入（距离和交通等级）
"""