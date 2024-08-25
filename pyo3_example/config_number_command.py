import argparse
import yaml
import os

from pprint import pprint

from .pyo3_example import run_config


def config_number_command():
    parser = argparse.ArgumentParser(description="calculate fibonacci numbers using a config file.")
    parser.add_argument("--path", action="store", type=str, required=True, help="path to config file")
    args = parser.parse_args()

    with open(os.getcwd() + "/" + args.path) as f:
        config_data: dict = yaml.safe_load(f)

        print("config data:")
        pprint(config_data)

        print("the result:")
        pprint(run_config(config_data))
