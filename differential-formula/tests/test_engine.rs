extern crate differential_formula;

use differential_formula::constraint::*;
use differential_formula::expression::*;
use differential_formula::term::*;
use differential_formula::engine::*;
use differential_formula::rule::*;

use differential_formula::composite;
use differential_formula::variable;
use differential_formula::atom;

use std::sync::Arc;
use std::rc::Rc;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/*
#[test]
fn test_ddengine() {
    let e11: Term = composite!{ 
        Edge(
            composite!{ Node(Atom::Int(0).into()) }, 
            composite!{ Node(Atom::Int(0).into()) }
        ) 
    };

    let e1: Term = composite!{ 
        Edge(
            composite!{ Node(Atom::Int(0).into()) }, 
            composite!{ Node(Atom::Int(1).into()) }
        ) 
    };

    let e2: Term = composite!{ 
        Edge(
            composite!{ Node(Atom::Int(1).into()) }, 
            composite!{ Node(Atom::Int(2).into()) }
        ) 
    };

    let e3: Term = composite!{ 
        Edge(
            composite!{ Node(Atom::Int(2).into()) }, 
            composite!{ Node(Atom::Int(3).into()) }
        ) 
    };


    let e4: Term = composite!{ 
        Edge(
            composite!{ Node(Atom::Int(3).into()) }, 
            composite!{ Node(Atom::Int(4).into()) }
        ) 
    };

    let node_a: Term = composite! { Node(variable!(a.)) };
    let node_b: Term = composite! { Node(variable!(b.)) };

    let edge_aa: Term = composite! { Edge(variable!(a.), variable!(b.)) };
    let edge_ab: Term = composite! { Edge(variable!(a.), variable!(b.)) };
    let edge_bc: Term = composite! { Edge(variable!(b.), variable!(c.)) };
    let edge_ac: Term = composite! { Edge(variable!(a.), variable!(c.)) };
    let edge_cd: Term = composite! { Edge(variable!(c.), variable!(d.)) };
    let edge_ad: Term = composite! { Edge(variable!(a.), variable!(d.)) };

    let path_ab: Term = composite! { Path(variable!(a.), variable!(b.)) };
    let path_ac: Term = composite! { Path(variable!(a.), variable!(c.)) };
    let path_uv: Term = composite! { Path(variable!(u.), variable!(v.)) };
    let path_uu: Term = composite! { Path(variable!(u.), variable!(u.)) };

    let nocycle_u: Term = composite! { NoCycle(variable!(u.)) };
    let line_abcd: Term = composite! { 
        Line(variable!(a.), variable!(b.), 
             variable!(c.), variable!(d.)
        ) 
    };
    let twoedge_xy: Term = composite! { TwoEdge(variable!(x.), variable!(y.)) };
    let twoedge_xx: Term = composite! { TwoEdge(variable!(x.), variable!(x.)) };
    let twoedge_xx_square: Term = composite! { TwoEdge(variable!(x.), variable!(x.), variable!(square.)) };
    
    // path(a, b) :- edge(a, b).
    let rule1 = Rule {
        head: vec![path_ab],
        body: vec![
            Predicate { negated: false, term: edge_ab.clone(), alias: None }.into(),
        ],
    };

    // path(a, c) :- edge(a, b), edge(b, c).
    let rule2 = Rule {
        head: vec![path_ac],
        body: vec![
            Predicate { negated: false, term: edge_ab.clone(), alias: None }.into(),
            Predicate { negated: false, term: edge_bc.clone(), alias: None }.into(),
        ],
    };

    // edge(a, c) :- edge(a, b), edge(b, c).
    let rule2x = Rule {
        head: vec![edge_ac],
        body: vec![
            Predicate { negated: false, term: edge_ab.clone(), alias: None }.into(),
            Predicate { negated: false, term: edge_bc.clone(), alias: None }.into(),
        ]
    };

    // edge(a, d) :- edge(a, b), edge(b, c), edge(c, d).
    let rule3 = Rule {
        head: vec![edge_ad],
        body: vec![
            Predicate { negated: false, term: edge_ab.clone(), alias: None }.into(),
            Predicate { negated: false, term: edge_bc.clone(), alias: None }.into(),
            Predicate { negated: false, term: edge_cd.clone(), alias: None }.into(),
        ],
    };

    // nocycle(u) :- path(u, v), no path(u, u).
    let rule4 = Rule {
        head: vec![nocycle_u],
        body: vec![
            Predicate { negated: false, term: path_uv, alias: None }.into(),
            Predicate { negated: true, term: path_uu, alias: None }.into(),
        ],
    };

    // Production rule: line(a, b, c, d) :- edge(a, b), edge(c, d).
    let rule5 = Rule {
        head: vec![line_abcd],
        body: vec![
            Predicate { negated: false, term: edge_ab.clone(), alias: None }.into(),
            Predicate { negated: false, term: edge_cd.clone(), alias: None }.into(),
        ],
    };


    // TwoEdge(x, y) :- x is Edge(a, b), y is Edge(b, c).
    let rule6 = Rule {
        head: vec![twoedge_xy],
        body: vec![
            Predicate { negated: false, term: edge_ab.clone(), alias: Some(variable!(x.)) }.into(),
            Predicate { negated: false, term: edge_bc.clone(), alias: Some(variable!(y.)) }.into(),
        ],
    };


    let s = SetComprehension {
        vars: vec![variable!(b.), edge_aa],
        condition: vec![
            Predicate { negated: false, term: edge_ab.clone(), alias: None }.into(),
        ],
        op: SetCompreOp::Count,
        default: None, 
    };

    let count_expr = Binary {
        op: BinOp::Eq,
        left: Expr::BaseExpr(BaseExpr::Term(variable!(aggr.))),
        right: Expr::BaseExpr(BaseExpr::SetComprehension(s)),
    };

    let bin_expr = Binary {
        op: BinOp::Eq,
        left: ArithExpr { 
            op: ArithmeticOp::Mul, 
            left: Arc::new(BaseExpr::Term(variable!(aggr.).into()).into()),
            right: Arc::new(BaseExpr::Term(Atom::Int(2).into()).into()),
        }.into(),
        right: BaseExpr::Term(Atom::Int(20).into()).into(),
    };

    let square_expr = Binary {
        op: BinOp::Eq,
        left: BaseExpr::Term(variable!(square.).into()).into(),
        right: ArithExpr {
            op: ArithmeticOp::Mul,
            left: Arc::new(BaseExpr::Term(variable!(aggr.).into()).into()),
            right: Arc::new(BaseExpr::Term(variable!(aggr.).into()).into()),
        }.into(),
    };

    // TwoEdge(x, x, square) :- x is Edge(c, d), aggr = count({Edge(a, a), b | Edge(a, b)}), 
    //                  square = aggr*aggr, aggr * 2 = 20.
    let rule7 = Rule {
        head: vec![twoedge_xx_square.clone()],
        body: vec![
            Predicate { negated: false, term: edge_cd.clone(), alias: variable!(x.).into() }.into(),
            count_expr.into(),
            square_expr.into(),
            bin_expr.into(),
        ],
    };

    println!("{:?}", rule7.derived_variables());

    // Each rule forms a stratum.
    //let mut engine = DDEngine::new(vec![], vec![rule2x, rule1, rule4]);
    //let mut engine = DDEngine::new(vec![], vec![rule2x, rule5]);
    //let mut engine = DDEngine::new(vec![], vec![rule6]);
    let mut engine = DDEngine::new(vec![], vec![rule7]);

    let (mut input, probe) = engine.create_dataflow();

    input.insert(e1);
    input.insert(e11);
    input.insert(e2);
    input.insert(e3.clone());
    input.insert(e4.clone());
    input.insert(e4);

    input.advance_to(1);
    input.flush();
    while probe.less_than(&input.time()) {
        engine.worker.step();
    }
    

    input.remove(e3);

    input.advance_to(2);
    input.flush();
    while probe.less_than(&input.time()) {
        engine.worker.step();
    }
        
}

*/