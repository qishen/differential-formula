use std::borrow::Borrow;
use std::convert::*;
use std::sync::*;
use std::iter::*;
use std::vec::Vec;
use std::collections::{BTreeMap, HashSet, HashMap};

use im::OrdSet;
use timely::dataflow::*;
use differential_dataflow::*;
use differential_dataflow::input::InputSession;
use differential_dataflow::operators::join::*;
use differential_dataflow::operators::*;
use differential_dataflow::hashable::*;

// Import from local ddlog repo and APIs are subject to constant changes
use differential_datalog::record::*;
use differential_datalog::program::{Rule as DDRule};

use crate::constraint::*;
use crate::term::*;
use crate::expression::*;
use crate::module::*;
use crate::rule::*;
use crate::parser::combinator::*;
use crate::util::*;
use crate::util::map::*;

struct DDLogEngine {

}


