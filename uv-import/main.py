from uv_lib import custom_sum
from rupy_uv._core import custom_divide, custom_sum as core_custom_sum2


def main():
    print("This is the main function.")
    print(custom_sum(1, 2))
    print("This is the custom divide function.")
    print(custom_divide(1, 2))
    print("This is the custom sum function.")
    print(core_custom_sum2(1, 2))


if __name__ == "__main__":
    main()
