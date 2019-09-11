from enum import Enum


class BinOp(Enum):
    MUL = 1
    DIV = 2
    MOD = 3
    PLUS = 4
    MINUS = 5

    def __str__(self):
        if self.value == 1:
            return '*'
        elif self.value == 2:
            return '/'
        elif self.value == 3:
            return 'mod'
        elif self.value == 4:
            return '+'
        elif self.value == 5:
            return '-'
        else:
            return 'Not supported.'


class RelOp(Enum):
    EQ = 1
    NEQ = 2
    LT = 3
    LE = 4
    GT = 5
    GE = 6

    def __str__(self):
        if self.value == 1:
            return '='
        elif self.value == 2:
            return '!='
        elif self.value == 3:
            return '<'
        elif self.value == 4:
            return '<='
        elif self.value == 5:
            return '>'
        elif self.value == 6:
            return '>='
        else:
            return 'Not supported.'


class ArithmeticExprNode:
    def __init__(self, left, right, op):
        # ArithmeticExprNode can be composed by ArithmeticExprNode, AggregationNode
        # VariableNode or ConstantNode.
        self.left = left
        self.right = right
        self.op = op
