import unittest

from plantr_parser import parse_tokens,TokenList


class TestParser(unittest.TestCase):
    def test_is(self):
        vars = dict()
        parse_tokens(TokenList(['[pot]', 'is', '4']), vars)
        self.assertEqual(vars['pot'], 4)

    def test_is_to_is(self):
        vars = dict()
        parse_tokens(TokenList(['[pot1]', 'is', '4']), vars)
        parse_tokens(TokenList(['[pot2]', 'is', '[pot1]']), vars)
        self.assertEqual(vars['pot2'], 4)

    def test_show(self):
        vars = dict()
        parse_tokens(TokenList(['[pot]', 'is', '4']), vars)
        parse_tokens(TokenList(['show', '[pot]']), vars)

    def test_add(self):
        vars = dict()
        parse_tokens(TokenList(['[pot]', 'is', '3', '#', '3']), vars)
        self.assertEqual(vars['pot'], 6)
    
    def test_error(self):
        vars = dict()
        with self.assertRaises(ValueError):
            parse_tokens(TokenList(['[pot]', 'is', 'a']), vars)

    def test_subtract(self):
        vars = dict()
        parse_tokens(TokenList(['[pot]', 'is', '3', '~', '1']), vars)
        self.assertEqual(vars['pot'], 2)
    
    def test_add_subtract(self):
        vars = dict()
        parse_tokens(TokenList(['[pot]', 'is', '3', '#', '1']), vars)
        parse_tokens(TokenList(['[pot]', 'is', '[pot]', '~', '1']), vars)
        self.assertEqual(vars['pot'], 3)
    
    def test_add_subtract_show(self):
        vars = dict()
        parse_tokens(TokenList(['[pot]', 'is', '3', '#', '1']), vars)
        parse_tokens(TokenList(['[pot]', 'is', '[pot]', '~', '1']), vars)
        parse_tokens(TokenList(['show', '[pot]']), vars)
        self.assertEqual(vars['pot'], 3)
