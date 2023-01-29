// edge trait

use crate::graph::types::node::Node;

use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::types::misc::EdgeType;

/// Promotes anything that implements [GraphObject] trait to [Edge]
pub trait Edge
where
    Self: GraphObject,
{
    /// start node of the edge
    fn start(&self) -> &Node;
    /// end node of the edge
    fn end(&self) -> &Node;
    /// type [Directed], [Undirected] of the edge
    fn has_type(&self) -> &EdgeType;
}
