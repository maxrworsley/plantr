import sys
import re

def raise_parse_error(variables, remaining, error_message):
    """Raise a parse error and print out the variables and remaining tokens"""
    print('Parse error')
    print('Variables:')
    print(variables)
    print('Remaining:')
    print(remaining)
    raise ValueError(f'Parse error - {error_message}')

def is_variable(token):
    return re.match(r'\[[a-zA-Z0-9]+\]', token) is not None

def is_number(token):
    return re.match(r'\d+', token) is not None

def is_show(token):
    return token == 'show'

def is_add(token):
    return token == '#'

def is_subtract(token):
    return token == '~'

def parse_tokens(tokens, variables):
    """parse the tokens, doing the actions described where [var] indicates a variable, # indicates addition and ~ indicates subtraction"""

    if len(tokens) == 0:
        raise_parse_error(variables, tokens, 'expected variable or number')

    focus = tokens[0]

    if is_variable(focus):
        variable_name  = focus[1:-1]
        if len(tokens) == 1:
            return variables[variable_name]
        elif tokens[1] == 'is':
            variables[variable_name] = parse_tokens(tokens[2:], variables)
        else:
            raise raise_parse_error(variables, tokens, 'expected "is"')
    elif is_show(focus):
        print(parse_tokens(tokens[1:], variables))
    elif is_number(focus):
        if len(tokens) == 1:
            return int(focus)
        elif is_add(tokens[1]):
            return int(focus) + parse_tokens(tokens[2:], variables)
        elif is_subtract(tokens[1]):
            return int(focus) - parse_tokens(tokens[2:], variables)
        else:
            raise raise_parse_error(variables, tokens, 'expected # or ~')
    else:
        raise raise_parse_error(variables, tokens, 'expected variable or number')
        

def parse_lines(lines):
    variables = {}
    for line in lines:
        tokens = line.split(' ')
        
        parse_tokens(tokens, variables)


def get_lines(file_name):
        """Read the file and return a list of lines"""
        with open(file_name, 'r') as file:
            return file.readlines()

if __name__ == "__main__":
    file_name = sys.argv[1]

    lines = get_lines(file_name)
    parse_lines(lines)