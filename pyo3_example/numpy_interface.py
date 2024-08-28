from .pyo3_example import calculate_times, calculate_parameters


class NumpyInterface:
    def __init__(self):
        self.inventory = {}

    def calc_times(self, distance, traffic_grade):
        result = calculate_times({}, distance, traffic_grade)
        self.inventory["car time"] = result["times"][0][0]
        self.inventory["trunk time"] = result["times"][1][0]

    def calc_parameters(self, car_time, trunk_time):
        result = calculate_parameters({}, car_time, trunk_time)
        self.inventory["distance"] = result["parameters"][0][0]
        self.inventory["traffic grade"] = result["parameters"][1][0]
