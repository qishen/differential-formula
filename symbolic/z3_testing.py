from z3 import *

def test_simplify():
    x, y = Reals('x y')
    # Put expression in sum-of-monomials form
    t = simplify((x + y)**4, som=True)
    print(t)

    t2 = simplify(t, mul_to_power=True)
    print(t2)


def test_treelist():

    Tree = Datatype('Tree')
    TreeList = Datatype('TreeList')

    Tree.declare('leaf', ('val', IntSort()))
    Tree.declare('node', ('left', TreeList), ('right', TreeList))

    TreeList.declare('nil')
    TreeList.declare('cons', ('car', Tree), ('cdr', TreeList))

    Tree, TreeList = CreateDatatypes(Tree, TreeList)

    # t1  = Tree.leaf(10)
    # tl1 = TreeList.cons(t1, TreeList.nil)
    # t2  = Tree.node(tl1, TreeList.nil)
    # print(t2)
    # print(Tree.val(t1))
    # print(simplify(Tree.val(t1)))

    s = Solver()
    t1, t2, t3 = Consts('t1 t2 t3', TreeList)
    s.add(Distinct(t1, t2, t3))
    s.add(t1 != t2)
    # solve(Distinct(t1, t2, t3))

    print(s.check())
    print(s.model())


def sorting_with_datatypes():
    # input ::= new (int, int, int, int)
    Input = Datatype("Input")
    Input.declare('input', ('one', IntSort()), ('two', IntSort()), ('three', IntSort()), ('four', IntSort()))
    Input = Input.create()

    # trace ::= new (int, int, int, int)
    Trace = Datatype("Trace")
    Trace.declare('input', ('one', IntSort()), ('two', IntSort()), ('three', IntSort()), ('four', IntSort()))
    Trace = Trace.create()

    # Suppose we have two input facts i1 and i2
    i1, i2 = Consts('i1 i2', Input)
    t1, t2 = Consts('t1 t2', Trace)

    s = Solver()
    s.add(Distinct(i1, i2))
    s.add(Distinct(t1, t2))

    # How to define constraints between instances of datatypes.

    print(s.check())
    model = s.model()
    print(model)