// graph trait
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::types::edge::E;
use crate::graph::types::node::V;

/// Promotes an object to being a graph.
/// This trait is the gateway for using all the graph related operations in
/// the library
pub trait Graph: GraphObject {
    /// outputs a [Node] set.
    /// a [Node] can constructed anything that implements the Node trait
    fn vertices<'a>(&'a self) -> V<'a>;

    /// outputs an [Edge] set.
    /// an [Edge] can constructed anything that implements the Edge trait
    fn edges<'a>(&'a self) -> E<'a>;
}
//
