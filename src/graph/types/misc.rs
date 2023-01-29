/// diverse graph related types
use crate::graph::traits::graph_obj::GraphObject;
use std::clone::Clone;
use std::collections::HashSet;
use std::fmt::Debug;
use std::iter::Iterator;
// edge type enum
use std::fmt;

/// Indicates whether an edge is directed or undirected.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum EdgeType {
    /// directed edge: it has implications on neighborhood functions
    Directed,
    /// undirected edge: it has implications on neighborhood functions
    Undirected,
}

impl fmt::Display for EdgeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct GraphObjects<T: GraphObject + Debug + Clone + PartialEq> {
    set: HashSet<T>,
}

impl<T: GraphObject + Debug + Clone + PartialEq> Iterator for GraphObjects<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let mut hs = self.set;
        let mut hiter = hs.iter();
        let item = hiter.next();
        match item {
            Some(i) => Some(i.clone()),
            None => None,
        }
    }
}

pub struct GraphContent<'a, T: GraphObject + Debug + Clone + PartialEq> {
    pub set: HashSet<&'a T>,
}

impl<'a, T: GraphObject + Debug + Clone + PartialEq> Iterator for GraphContent<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let mut hs = self.set;
        let mut hiter = hs.iter();
        let item = hiter.next();
        match item {
            Some(i) => Some(i.clone()),
            None => None,
        }
    }
}

impl<T: GraphObject + Debug + Clone + PartialEq> GraphObjects<T> {
    pub fn new(objs: HashSet<T>) -> GraphObjects<T> {
        GraphObjects { set: objs }
    }
    pub fn from_contents(gs: GraphContent<T>) -> GraphObjects<T> {
        let mut hs = HashSet::new();
        for g in gs.set {
            hs.insert(g.clone());
        }
        GraphObjects { set: hs }
    }
    pub fn to_contents<'a>(&self) -> GraphContent<'a, T> {
        let mut hs = HashSet::new();
        for e in self.set {
            hs.insert(&e.clone());
        }
        GraphContent { set: hs }
    }
}

impl<'a, T: GraphObject + PartialEq> GraphContent<'a, T> {
    pub fn new(objs: HashSet<&'a T>) -> GraphContent<'a, T> {
        GraphContent { set: objs }
    }
    pub fn from_objects(objs: GraphObjects<T>) -> GraphContent<'a, T> {
        objs.to_contents()
    }
    pub fn to_objects(&self) -> GraphObjects<T> {
        let mut hs = HashSet::new();
        for e in self.set {
            hs.insert(e.clone());
        }
        GraphObjects::new(hs)
    }
}
