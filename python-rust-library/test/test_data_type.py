import unittest


class TestDataType(unittest.TestCase):
    def test_data_type(self):
        from digits_pi import data_types
        rust_data = data_types()
        self.assertIsInstance(rust_data, dict)
        data = {"int": 42, "float": 3.14, "str": "Hello, World!", "bool": True, "list": [1, 2, 3], "tuple": (1, 2, 3),
                "set": {1, 2, 3}, "dict": {1: "one", 2: "two"}}
        self.assertEqual(rust_data, data)


if __name__ == '__main__':
    unittest.main()
