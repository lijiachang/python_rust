import argparse
from .pyo3_example import fibonacci_number


def fib_number_command() -> None:
    parser = argparse.ArgumentParser(description="calculate fibonacci numbers")
    parser.add_argument("--number", action="store", type=int, required=True, help="fibonacci number to be calculate")
    args = parser.parse_args()

    print(f"your fibonacci number is:{fibonacci_number(n=args.number)} ")
