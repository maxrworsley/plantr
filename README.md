# Plantr

Plantr is an interpreted language, made with the goal of testing out GitHub Copilot alongside a normal development workflow.

## Table of Contents

- [Syntax](#Syntax)
- [Usage](#Usage)

## Syntax
The syntax for Plantr consists of 4 components:
- ```[var]``` to indicate a variable called *var*
- ```[var] is <x>``` to set a variable equal to x
- ```<a> # <b>``` to indicate the values of a and b added together. They can be either numbers or variables
- ```<a> ~ <b>``` to indicate the values of a and b subtracted. They can be either numbers or variables
- ```show <a>``` to print the value of a, either a number or variable
- ```// Comment``` to indicate a comment. Any further input on that line is igorned


### Example
```
[potA] is 2
[potB] is 3

[potC] is [potA] # [potB]

[potD] is [potC] ~ [potA]

show [potD] // Prints out 3
```

The example above defines four variables, potA, potB, potC, and potD, and performs some operations on them.

The first two lines of the example assign the values 2 and 3 to the variables potA and potB, respectively. The next line assigns the value of potA + potB to the variable potC.

The fourth line assigns the value of potC - potA to the variable potD.

Finally, the example outputs the value of potD using the show keyword. The expected output of the example is 3, which is the value of potD after the operations have been performed. This will be sent to stdout.

## Usage

To use the parser, you can use the function as an import or run it directly on a file.

### As an import
```python
from plantr_parser import parse_lines, get_lines

lines = get_lines(file_path)
parse_lines(lines)
```

### From a shell
```cmd
python plantr_parser.py <file_path>
```

