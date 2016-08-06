use libc::{c_char, size_t};

use {TF_Buffer, TF_Node, TF_Status};

#[derive(Clone, Copy, Debug)]
pub enum TF_Graph {}

extern {
    pub fn TF_NewGraph() -> *mut TF_Graph;
    pub fn TF_DeleteGraph(graph: *mut TF_Graph);
    pub fn TF_GraphNodeByName(graph: *mut TF_Graph, name: *const c_char) -> *mut TF_Node;
    pub fn TF_GraphNextNode(graph: *mut TF_Graph, position: *mut size_t) -> *mut TF_Node;
    pub fn TF_GraphToGraphDef(graph: *mut TF_Graph, definition: *mut TF_Buffer,
                              status: *mut TF_Status);
}
