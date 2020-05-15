use differential_dataflow::input::{Input, InputSession};
use differential_dataflow::operators::join::{Join, JoinCore};
use differential_dataflow::operators::*;

use z3::ast::Ast;
use z3::*;

fn main() {
    test_solver();
    test_dd();
}

fn test_solver() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let a = ast::BV::new_const(&ctx, "a", 64);
    let b = ast::BV::new_const(&ctx, "b", 64);
    let two = ast::BV::from_i64(&ctx, 2, 64);

    let solver = Solver::new(&ctx);
    solver.assert(&a.bvsgt(&b)); // a > b
    solver.assert(&b.bvsgt(&two)); // a > 2
    let b_plus_two = b.bvadd(&two); 
    solver.assert(&b_plus_two.bvsgt(&a)); // b + 2 > a
    assert_eq!(solver.check(), SatResult::Sat);

    let model = solver.get_model();
    let av = model.eval(&a).unwrap().as_i64().unwrap();
    let bv = model.eval(&b).unwrap().as_i64().unwrap();
    assert!(av > bv);
    assert!(bv > 2);
    assert!(bv + 2 > av);
    println!("{:?}, {:?}", av, bv);
}

fn test_dd() {
    timely::execute_from_args(std::env::args(), move |worker| {
        // create an input collection of data.
        let mut input = InputSession::new();
        let mut input2 = InputSession::new();

        // define a new computation.
        worker.dataflow(|scope| { 
            let edge1 = input.to_collection(scope)
                .inspect(|x| println!("Input 1: {:?}", x));
            let edge2 = input2.to_collection(scope)
                .inspect(|x| println!("Input 2: {:?}", x));

            edge1
                .join(&edge2)
                .inspect(|x| println!("Join results: {:?}", x));
        });

        for i in 0 .. 5 {
            input.insert((i, i+1));
            input2.insert((i, i));
        }
        input.advance_to(1);

        input.insert((4, 100));
        input2.insert((4, 200));
        input.advance_to(2);

        input.remove((4, 100));
        input.advance_to(3);

    }).unwrap();
}