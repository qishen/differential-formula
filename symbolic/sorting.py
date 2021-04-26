from z3 import *

"""
Problem definition:
Try to find a list of four ints that cannot be sorted by swapping two adjacent items as 
counter example

The problem is modeled in FORMULA with recursion, negation and infinite integer domain.
A naive solution is to run concrete execution in every input, which is not possible because
the integer domain is infinite. We run symbolic execution with symbolic values and dump the
intermediate results into z3 solver to prove the absence of counter example.

domain Sorting {

    input ::= new (Integer, Integer, Integer, Integer).
    trace ::= (Integer, Integer, Integer, Integer).
    cntrexmp ::= (Integer, Integer, Integer, Integer).

    cntrexmp(W, X, Y, Z) :- input(W, X, Y, Z), no { A, B, C, D | trace(A, B, C, D), A <= B, B <= C, C <= D }.

    trace(W, X, Y, Z) :- input(W, X, Y, Z).
    trace(X, W, Y, Z) :- trace(W, X, Y, Z), W > X.
    trace(W, Y, X, Z) :- trace(W, X, Y, Z), X > Y.
    trace(W, X, Z, Y) :- trace(W, X, Y, Z), Y > Z.
}

// Define a partial model with only one fact. We can define more facts but one is more than enough
// to generate counter example if it exists.
partial model of Sorting {
    input(s1, s2, s3, s4).
}

"""

def sorting_symbexec_single_step():
    """
    A open world query: query(Program, Trace(x, y, x, y))
    1. Start symbolic execution from one initial fact with symbolic values Input(s1, s2, s3, s4).
    2. t1 = Trace(s1, s2, s3, s4) is inferred from the first rule.
    3. if s1 > s2, the second rule is triggered to generate new fact t2 = Trace(s2, s1, s3, s4) with condition s1 > s2.
    from one initial fact we can generate 3 different traces with different conditions.
    4. For new trace fact t2 = Trace(s2, s1, s3, s4), if s1 > s3 then t3 = Trace(s2, s3, s1, s4) with the condition
    s1 > s2 and s1 > s3. 
    5. Check the following formula in z3: 
    (s1 == s3 and s2 == s4) or (s1 > s2 and s2 == s3 and s1 == s4)

    The question is how many symbolic fixpoints exist and how far we want to expand the tree?
    """

    s1, s2, s3, s4 = Ints('s1 s2 s3 s4')
    s = Solver()
    # Execute rules only once with 3 possibilities.
    s.add(
        Or(
            And(s1 == s3, s2 == s4),
            And(s1 > s2, s2 == s3, s1 == s4),
            And(s2 > s3, s1 == s2, s3 == s4),
            And(s3 > s4, s1 == s4, s2 == s3)
        )
    )

    print(s.check())
    model = s.model()
    print(model)


class SortingFixedPointState:
    trace = []
    constraints = set()

    def __init__(self, trace, constraints):
        self.trace = trace
        self.constraints = constraints

    def __eq__(self, other):
        return self.trace == other.trace and self.constraints == other.constraints

    def __hash__(self):
        return hash((tuple(self.trace), tuple(self.constraints)))

    def __str__(self):
        trace_inner = ", ".join([str(i) for i in self.trace])
        contraint_inner = ", ".join([str(i) for i in self.constraints])
        return "trace({}), [{}]".format(trace_inner, contraint_inner)

    def execute(self):
        states = []
        for i in range(0, len(self.trace)-1):
            trace = list(self.trace)
            # Abandon the state if (a < b) exists in constraints while we want to add (a > b).
            # TODO: Check other conflicts in the constraint set.
            if (trace[i+1] > trace[i]) not in self.constraints:
                new_constraint = trace[i] > trace[i+1]
                tmp = trace[i]
                trace[i] = trace[i+1]
                trace[i+1] = tmp
                constraints = set(self.constraints)
                constraints.update([new_constraint])
                new_state = SortingFixedPointState(trace, constraints)
                states.append(new_state)
        return states
        

def sorting_symbexec():
    # Check if a counter example cntrexmp(W, X, Y, Z) exists or the absence of trace(A, B, C, D) 
    # where A <= B, B <= C, C <= D.
    # cntrexmp(W, X, Y, Z) :- input(W, X, Y, Z), no { A, B, C, D | trace(A, B, C, D), A <= B, B <= C, C <= D }.
    # trace(W, X, Y, Z) :- input(W, X, Y, Z).
    # trace(X, W, Y, Z) :- trace(W, X, Y, Z), W > X.
    # trace(W, Y, X, Z) :- trace(W, X, Y, Z), X > Y.
    # trace(W, X, Z, Y) :- trace(W, X, Y, Z), Y > Z.

    s1, s2, s3, s4 = Ints('s1 s2 s3 s4')
    s = Solver()
    state_set = set()
    stack = []
    initial_trace = [s1, s2, s3, s4]
    initial_state = SortingFixedPointState(initial_trace, [])
    state_set.add(initial_state)
    stack.append(initial_state)

    while len(stack) != 0:
        state = stack.pop()
        for new_state in state.execute():
            if new_state not in state_set:
                stack.append(new_state)
                state_set.add(new_state)

    # Find a input that eventually outputs a sorted trace after symbolic execution.
    conjunctions = [] 
    for state in state_set:
        trace = state.trace
        # where A <= B, B <= C, C <= D in trace(A, B, C, D).
        goal_constraints = []
        for i in range(0, len(trace)-1):
            goal_constraints.append(trace[i] < trace[i+1])
        conjunctions.append(And(list(state.constraints) + goal_constraints))
        print(state)

    # What if we want to find counter example to prove the absence of sorted trace after
    # symbolic execution?

    # Find an assignment for s1, s2, s3, s4 that the traces from symbolic execution reach the goal.
    # s.add(Or(conjunctions))
    
    # Find an assignment that won't reach the goal and it is not possible.
    # s.add(Not(Or(conjunctions)))

    # Oops, they have to be distinct otherwise the solver may assign same value to multiple variables to 
    # escape the constraints we set.
    # s.add([Distinct(s1, s2, s3, s4), Not(Or(conjunctions))])

    # For all possible assignments to s1, s2, s3, s4, all of them will reach the goal.
    # s.add(ForAll([s1, s2, s3, s4], [Distinct(s1, s2, s3, s4), Or(conjunctions)]))

    # Check if there exists an assignment that fails to reach the goal.
    s.add(Exists([s1, s2, s3, s4], 
        And(Distinct(s1, s2, s3, s4), Not(Or(conjunctions)))
    ))

    print("Constraint generation finished and start model generation...")
    print(s.check())
    if s.check() == sat:
        model = s.model()
        print(model)




# Check if a counter example cntrexmp(W, X, Y, Z) exists or the absence of trace(A, B, C, D) 
# where A <= B, B <= C, C <= D.
# cntrexmp(W, X, Y, Z) :- input(W, X, Y, Z), no { A, B, C, D | trace(A, B, C, D), A <= B, B <= C, C <= D }.
# trace(W, X, Y, Z) :- input(W, X, Y, Z).
# trace(X, W, Y, Z) :- trace(W, X, Y, Z), W > X.
# trace(W, Y, X, Z) :- trace(W, X, Y, Z), X > Y.
# trace(W, X, Z, Y) :- trace(W, X, Y, Z), Y > Z.

# input ::= new (int, int, int, int)
Input = Datatype("Input")
Input.declare('input', ('one', IntSort()), ('two', IntSort()), ('three', IntSort()), ('four', IntSort()))
Input = Input.create()
# trace ::= new (int, int, int, int)
Trace = Datatype("Trace")
Trace.declare('trace', ('one', IntSort()), ('two', IntSort()), ('three', IntSort()), ('four', IntSort()))
Trace = Trace.create()

class SortingFPDatatypeState:
    variables = []
    trace = None
    constraints = set()

    def __init__(self, variables, constraints):
        self.constraints = constraints
        self.variables = variables
        self.trace = Trace.trace(self.variables)

    def __eq__(self, other):
        return self.trace == other.trace and self.constraints == other.constraints

    def __hash__(self):
        return hash((self.trace, tuple(self.constraints)))

    def __str__(self):
        contraint_inner = ", ".join([str(i) for i in self.constraints])
        return "{}, [{}]".format(self.trace, contraint_inner)

    def execute(self):
        states = []
        for i in range(0, len(self.variables)-1):
            variables = list(self.variables)
            # Abandon the state if (a < b) exists in constraints while we want to add (a > b).
            # TODO: Check other conflicts in the constraint set.
            if (variables[i+1] > variables[i]) not in self.constraints:
                new_constraint = variables[i] > variables[i+1]
                tmp = variables[i]
                variables[i] = variables[i+1]
                variables[i+1] = tmp
                constraints = set(self.constraints)
                constraints.update([new_constraint])
                new_state = SortingFPDatatypeState(variables, constraints)
                states.append(new_state)
        return states

def sorting_symbexec_datatype():
    s1, s2, s3, s4 = Ints('s1 s2 s3 s4')
    initial_state = SortingFPDatatypeState([s1, s2, s3, s4], [])

    state_set = set()
    stack = []
    state_set.add(initial_state)
    stack.append(initial_state)

    while len(stack) != 0:
        state = stack.pop()
        for new_state in state.execute():
            if new_state not in state_set:
                stack.append(new_state)
                state_set.add(new_state)

    # Find a input that eventually outputs a sorted trace after symbolic execution.
    conjunctions = [] 
    for state in state_set:
        variables = state.variables
        # where A <= B, B <= C, C <= D in trace(A, B, C, D).
        goal_constraints = []
        for i in range(0, len(variables)-1):
            goal_constraints.append(variables[i] < variables[i+1])
        conjunctions.append(And(list(state.constraints) + goal_constraints))
        print(state)

    # TODO: How to access members inside trace?
    # print(initial_trace['one'])

    s = Solver()
    # Check if there exists an assignment that fails to reach the goal.
    # The goal is to find four distinct integers that all derived symbolic states 
    # from it do not stand for the goal 
    s.add(Exists([s1, s2, s3, s4], 
        And(Distinct(s1, s2, s3, s4), Not(Or(conjunctions)))
    ))

    # Find an assignment for s1, s2, s3, s4 that the traces from symbolic execution reach the goal.
    # s.add(Or(conjunctions))

    print("Constraint generation finished and start model generation...")
    print(s.check())
    if s.check() == sat:
        model = s.model()
        print(model)

sorting_symbexec()
sorting_symbexec_datatype()