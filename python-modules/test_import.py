import unittest


class TestingProperImportingAndExporting(unittest.TestCase):
    def test_subcommand_loading(self):
        try:
            import modules
            modules.sub
            result = modules.sub.command.run()
            self.assertTrue(result)
        except ImportError as e:
            self.fail("ImportError: " + str(e))
            
    def test_constants_loading(self):
        try:
            import modules
            result = modules.FOO
            self.assertTrue(result)
        except ImportError as e:
            self.fail("ImportError: " + str(e))


if __name__ == '__main__':
    unittest.main()
