use libc::{c_char, c_float, c_int, c_uchar, c_void, int64_t, size_t};

use {TF_Buffer, TF_DataType, TF_Graph, TF_Status, TF_Tensor};

#[derive(Clone, Copy, Debug)]
pub enum TF_NodeDescription {}

#[derive(Clone, Copy, Debug)]
pub enum TF_Node {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TF_Port {
    pub node: *mut TF_Node,
    pub index: c_int,
}

extern {
    pub fn TF_NewNode(graph: *mut TF_Graph, operation_type: *const c_char, name: *const c_char)
                      -> *mut TF_NodeDescription;
    pub fn TF_SetDevice(description: *mut TF_NodeDescription, device: *const c_char);
    pub fn TF_AddInput(description: *mut TF_NodeDescription, input: TF_Port);
    pub fn TF_AddInputList(description: *mut TF_NodeDescription, inputs: *const TF_Port,
                           ninputs: c_int);
    pub fn TF_AddControlInput(description: *mut TF_NodeDescription, input: *mut TF_Node);
    pub fn TF_SetAttrString(description: *mut TF_NodeDescription, name: *const c_char,
                            value: *const c_void, length: c_int);
    pub fn TF_SetAttrStringList(description: *mut TF_NodeDescription, name: *const c_char,
                                values: *const *const c_void, lengths: *const c_int,
                                nvalues: c_int);
    pub fn TF_SetAttrInt(description: *mut TF_NodeDescription, name: *const c_char,
                         value: int64_t);
    pub fn TF_SetAttrIntList(description: *mut TF_NodeDescription, name: *const c_char,
                             values: *const int64_t, nvalues: c_int);
    pub fn TF_SetAttrFloat(description: *mut TF_NodeDescription, name: *const c_char,
                           value: c_float);
    pub fn TF_SetAttrFloatList(description: *mut TF_NodeDescription, name: *const c_char,
                               values: *const c_float, nvalues: c_int);
    pub fn TF_SetAttrBool(description: *mut TF_NodeDescription, name: *const c_char,
                          value: c_uchar);
    pub fn TF_SetAttrBoolList(description: *mut TF_NodeDescription, name: *const c_char,
                              values: *const c_uchar, nvalues: c_int);
    pub fn TF_SetAttrType(description: *mut TF_NodeDescription, name: *const c_char,
                          value: TF_DataType);
    pub fn TF_SetAttrTypeList(description: *mut TF_NodeDescription, name: *const c_char,
                              values: *const TF_DataType, nvalues: c_int);
    pub fn TF_SetAttrShape(description: *mut TF_NodeDescription, name: *const c_char,
                           dims: *const int64_t, ndims: c_int);
    pub fn TF_SetAttrShapeList(description: *mut TF_NodeDescription, name: *const c_char,
                               dims: *const *const int64_t, ndims: *const c_int, nshapes: c_int);
    pub fn TF_SetAttrTensorShapeProto(description: *mut TF_NodeDescription, name: *const c_char,
                                      proto: *mut c_void, proto_length: c_int,
                                      status: *mut TF_Status);
    pub fn TF_SetAttrTensorShapeProtoList(description: *mut TF_NodeDescription,
                                          name: *const c_char, protos: *const *const c_void,
                                          proto_lengths: *const c_int, nshapes: c_int,
                                          status: *mut TF_Status);
    pub fn TF_SetAttrTensor(description: *mut TF_NodeDescription, name: *const c_char,
                            value: *mut TF_Tensor, status: *mut TF_Status);
    pub fn TF_SetAttrTensorList(description: *mut TF_NodeDescription, name: *const c_char,
                                values: *const *mut TF_Tensor, nvalues: c_int,
                                status: *mut TF_Status);
    pub fn TF_SetAttrToAttrValueProto(description: *mut TF_NodeDescription, name: *const c_char,
                                      proto: *const c_void, proto_length: size_t,
                                      status: *mut TF_Status);
    pub fn TF_FinishNode(description: *mut TF_NodeDescription, status: *mut TF_Status)
                         -> *mut TF_Node;
}

extern {
    pub fn TF_NodeName(node: *mut TF_Node) -> *const c_char;
    pub fn TF_NodeOpType(node: *mut TF_Node) -> *const c_char;
    pub fn TF_NodeDevice(node: *mut TF_Node) -> *const c_char;
    pub fn TF_NodeNumOutputs(node: *mut TF_Node) -> c_int;
    pub fn TF_NodeOutputType(output: TF_Port) -> TF_DataType;
    pub fn TF_NodeOutputListLength(node: *mut TF_Node, name: *const c_char, status: *mut TF_Status)
                                   -> c_int;
    pub fn TF_NodeNumInputs(node: *mut TF_Node) -> c_int;
    pub fn TF_NodeInputType(input: TF_Port) -> TF_DataType;
    pub fn TF_NodeInputListLength(node: *mut TF_Node, name: *const c_char, status: *mut TF_Status)
                                  -> c_int;
    pub fn TF_NodeInput(input: TF_Port) -> TF_Port;
    pub fn TF_NodeOutputNumConsumers(output: TF_Port) -> c_int;
    pub fn TF_NodeOutputConsumers(output: TF_Port, consumers: *mut TF_Port, max_consumers: c_int)
                                  -> c_int;
    pub fn TF_NodeNumControlInputs(node: *mut TF_Node) -> c_int;
    pub fn TF_NodeGetControlInputs(node: *mut TF_Node, control_inputs: *mut *mut TF_Node,
                                   max_control_inputs: c_int) -> c_int;
    pub fn TF_NodeNumControlOutputs(node: *mut TF_Node) -> c_int;
    pub fn TF_NodeGetControlOutputs(node: *mut TF_Node, control_outputs: *mut *mut TF_Node,
                                    max_control_outputs: c_int) -> c_int;
    pub fn TF_NodeGetAttrValueProto(node: *mut TF_Node, name: *const c_char, value: *mut TF_Buffer,
                                    status: *mut TF_Status);
    pub fn TF_NodeToNodeDef(node: *mut TF_Node, definition: *mut TF_Buffer,
                            status: *mut TF_Status);
}
