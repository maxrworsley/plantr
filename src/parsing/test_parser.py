import unittest

from plantr_parser import parse_tokens


class TestParser(unittest.TestCase):
    def test_is(self):
        vars = dict()
        parse_tokens(['[pot]', 'is', '4'], vars)
        self.assertEqual(vars['pot'], 4)

    def test_is_to_is(self):
        vars = dict()
        parse_tokens(['[pot1]', 'is', '4'], vars)
        parse_tokens(['[pot2]', 'is', '[pot1]'], vars)
        self.assertEqual(vars['pot2'], 4)

    def test_show(self):
        vars = dict()
        parse_tokens(['[pot]', 'is', '4'], vars)
        parse_tokens(['show', '[pot]'], vars)

    def test_add(self):
        vars = dict()
        parse_tokens(['[pot]', 'is', '3', '#', '3'], vars)
        self.assertEqual(vars['pot'], 6)
    
    def test_error(self):
        vars = dict()
        with self.assertRaises(ValueError):
            parse_tokens(['[pot]', 'is', 'a'], vars)

    
