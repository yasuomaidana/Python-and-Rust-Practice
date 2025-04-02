import unittest


class TestDigitsPi(unittest.TestCase):
    def test_loading(self):
        try:
            import digits_pi
        except ImportError:
            self.fail("Could not import digits_pi.py")

    def test_accuracy(self):
        from digits_pi import calculate_pi
        self.assertAlmostEqual(calculate_pi(250), 3.14, places=3)
        
    def test_divide(self):
        from digits_pi import divide
        with self.assertRaises(ZeroDivisionError):
            divide(1,0)
        self.assertEqual(divide(1, 2), 0.5)


if __name__ == '__main__':
    unittest.main()
