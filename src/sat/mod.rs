use sat::formula::{Var, Lit};

pub mod dimacs;
pub mod formula;
pub mod minisat;


#[derive(Default, Debug)]
pub struct Stats {
    pub solves        : u64,
    pub restarts      : u64,
    pub decisions     : u64,
    pub rnd_decisions : u64,
    pub conflicts     : u64,
    pub propagations  : u64,
    pub tot_literals  : u64,
    pub del_literals  : u64
}


pub enum SolveRes<Solver> {
    UnSAT(Stats),
    SAT(Vec<Lit>, Stats),
    Interrupted(f64, Solver)
}


pub trait Solver : Sized {
    fn nVars(&self) -> usize;
    fn nClauses(&self) -> usize;
    fn newVar(&mut self, upol : Option<bool>, dvar : bool) -> Var;
    fn addClause(&mut self, clause : &[Lit]) -> bool;
    fn preprocess(&mut self, &minisat::budget::Budget) -> bool;
    fn solveLimited(self, &minisat::budget::Budget, &[Lit]) -> SolveRes<Self>;
    fn stats(&self) -> Stats;
}
