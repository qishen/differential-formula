from enum import Enum


class BinOp(Enum):
    MUL = 1
    DIV = 2
    MOD = 3
    PLUS = 4
    MINUS = 5


class RelOp(Enum):
    EQ = 1
    NEQ = 2
    LT = 3
    LE = 4
    GT = 5
    GE = 6


class ArithmeticExprNode:
    def __init__(self, left, right, op):
        # ArithmeticExprNode can be composed by ArithmeticExprNode, AggregationNode
        # VariableNode or ConstantNode.
        self.left = left
        self.right = right
        self.op = op
