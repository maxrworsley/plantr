import sys
import re

class TokenList:
    def __init__(self, tokens) -> None:
        self.tokens = tokens
        self.index = 0

    def peek(self):
        return self.tokens[self.index]
    
    def consume(self):
        token = self.tokens[self.index]
        self.index += 1
        return token
    
    def is_empty(self):
        return len(self) == 0
    
    def __len__(self):
        return len(self.tokens) - self.index
    
    def __str__(self):
        return str(self.tokens[self.index:])
    

def raise_parse_error(variables, remaining, error_message):
    """Raise a parse error and print out the variables and remaining tokens"""
    print('Parse error')
    print('Variables:')
    print(variables)
    print('Remaining:')
    print(remaining)
    raise ValueError(f'Parse error - {error_message}')

is_variable = lambda x: re.match(r'\[[a-zA-Z0-9]+\]', x) is not None
is_number = lambda x: re.match(r'\d+', x) is not None
is_show = lambda x: x == 'show'
is_is = lambda x: x == 'is'
is_add = lambda x: x == '#'
is_subtract = lambda x: x == '~'
is_comment = lambda x: x == '//'

def parse_tokens(tokens: TokenList, variables):
    """parse the tokens, doing the actions described where [var] indicates a variable, # indicates addition and ~ indicates subtraction"""

    if tokens.is_empty():
        raise_parse_error(variables, tokens, 'expected variable or number')

    focus = tokens.consume()

    if is_comment(focus):
        return

    if is_variable(focus):
        variable_name = focus[1:-1]
        if tokens.is_empty() or is_comment(tokens.peek()):
            return variables[variable_name]
        
        next_token = tokens.consume()

        if is_is(next_token):
            variables[variable_name] = parse_tokens(tokens, variables)
        elif is_add(next_token):
            return variables[variable_name] + parse_tokens(tokens, variables)
        elif is_subtract(next_token):
            return variables[variable_name] - parse_tokens(tokens, variables)
        else:
            raise raise_parse_error(variables, tokens, 'expected is, # or ~')
    elif is_show(focus):
        print(parse_tokens(tokens, variables))

    elif is_number(focus):
        if len(tokens) == 0:
            return int(focus)
        
        next_token = tokens.consume()
        if is_add(next_token):
            return int(focus) + parse_tokens(tokens, variables)
        elif is_subtract(next_token):
            return int(focus) - parse_tokens(tokens, variables)
        else:
            raise raise_parse_error(variables, tokens, 'expected # or ~')
        
    else:
        raise raise_parse_error(variables, tokens, 'expected variable or number')
        

def parse_lines(lines):
    variables = {}
    for line in lines:
        tokens = list(filter(lambda x: x != '', line.strip().split(' ')))
        if tokens:
            token_list = TokenList(tokens)
            parse_tokens(token_list, variables)


def get_lines(file_name):
        """Read the file and return a list of lines"""
        with open(file_name, 'r') as file:
            return file.readlines()

if __name__ == "__main__":
    file_name = sys.argv[1]

    lines = get_lines(file_name)
    parse_lines(lines)