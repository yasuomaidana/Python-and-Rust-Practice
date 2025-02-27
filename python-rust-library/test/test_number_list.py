import unittest
from digits_pi import NumbersList


class TestNumberList(unittest.TestCase):

    def test_initialization(self):
        try:
            NumbersList(10)
            self.fail("Could not create NumbersList object")
        except Exception:
            self.assertTrue(True)

    def test_small_list(self):
        n = NumbersList()
        n.add(10)
        n.add(20)
        n.add(30)
        self.assertEqual(n.sum(), 60)

    def test_large_list(self):
        n = NumbersList()
        for i in range(1000):
            n.add(i)
        self.assertEqual(n.sum(), 499500)


if __name__ == '__main__':
    unittest.main()
