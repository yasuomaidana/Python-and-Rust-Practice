from unittest import TestCase
from to_call.single_function import simple_sum

class Test(TestCase):
    def test_simple_sum(self):
        self.assertEqual(simple_sum(1, 2), 3)
