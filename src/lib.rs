//! Binding to [TensorFlow][1].
//!
//! [1]: https://www.tensorflow.org

#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_char, c_float, c_int, c_uchar, c_void, int64_t, size_t};

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TF_DataType {
    TF_FLOAT = 1,
    TF_DOUBLE = 2,
    TF_INT32 = 3,
    TF_UINT8 = 4,
    TF_INT16 = 5,
    TF_INT8 = 6,
    TF_STRING = 7,
    TF_COMPLEX64 = 8,
    TF_INT64 = 9,
    TF_BOOL = 10,
    TF_QINT8 = 11,
    TF_QUINT8 = 12,
    TF_QINT32 = 13,
    TF_BFLOAT16 = 14,
    TF_QINT16 = 15,
    TF_QUINT16 = 16,
    TF_UINT16 = 17,
    TF_COMPLEX128 = 18,
    TF_HALF = 19,
}
pub use TF_DataType::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TF_Code {
    TF_OK = 0,
    TF_CANCELLED = 1,
    TF_UNKNOWN = 2,
    TF_INVALID_ARGUMENT = 3,
    TF_DEADLINE_EXCEEDED = 4,
    TF_NOT_FOUND = 5,
    TF_ALREADY_EXISTS = 6,
    TF_PERMISSION_DENIED = 7,
    TF_UNAUTHENTICATED = 16,
    TF_RESOURCE_EXHAUSTED = 8,
    TF_FAILED_PRECONDITION = 9,
    TF_ABORTED = 10,
    TF_OUT_OF_RANGE = 11,
    TF_UNIMPLEMENTED = 12,
    TF_INTERNAL = 13,
    TF_UNAVAILABLE = 14,
    TF_DATA_LOSS = 15,
}
pub use TF_Code::*;

#[derive(Clone, Copy, Debug)]
pub enum TF_Status {}

extern {
    pub fn TF_NewStatus() -> *mut TF_Status;
    pub fn TF_DeleteStatus(status: *mut TF_Status);
    pub fn TF_SetStatus(status: *mut TF_Status, code: TF_Code, message: *const c_char);
    pub fn TF_GetCode(status: *const TF_Status) -> TF_Code;
    pub fn TF_Message(status: *const TF_Status) -> *const c_char;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TF_Buffer {
    pub data: *const c_void,
    pub length: size_t,
    pub data_deallocator: Option<unsafe extern fn(*mut c_void, size_t)>,
}

extern {
    pub fn TF_NewBufferFromString(proto: *const c_void, proto_length: size_t) -> *mut TF_Buffer;
    pub fn TF_NewBuffer() -> *mut TF_Buffer;
    pub fn TF_DeleteBuffer(buffer: *mut TF_Buffer);
    pub fn TF_GetBuffer(buffer: *mut TF_Buffer) -> TF_Buffer;
}

#[derive(Clone, Copy, Debug)]
pub enum TF_Tensor {}

extern {
    pub fn TF_NewTensor(data_type: TF_DataType, dims: *const int64_t, ndims: c_int,
                        data: *mut c_void, length: size_t,
                        deallocator: Option<unsafe extern fn(*mut c_void, size_t, *mut c_void)>,
                        deallocator_argument: *mut c_void) -> *mut TF_Tensor;
    pub fn TF_DeleteTensor(tensor: *mut TF_Tensor);
    pub fn TF_TensorType(tensor: *const TF_Tensor) -> TF_DataType;
    pub fn TF_NumDims(tensor: *const TF_Tensor) -> c_int;
    pub fn TF_Dim(tensor: *const TF_Tensor, index: c_int) -> int64_t;
    pub fn TF_TensorByteSize(tensor: *const TF_Tensor) -> size_t;
    pub fn TF_TensorData(tensor: *const TF_Tensor) -> *mut c_void;
}

#[derive(Clone, Copy, Debug)]
pub enum TF_SessionOptions {}

extern {
    pub fn TF_NewSessionOptions() -> *mut TF_SessionOptions;
    pub fn TF_SetTarget(options: *mut TF_SessionOptions, target: *const c_char);
    pub fn TF_SetConfig(options: *mut TF_SessionOptions, proto: *const c_void,
                        proto_length: size_t, status: *mut TF_Status);
    pub fn TF_DeleteSessionOptions(options: *mut TF_SessionOptions);
}

#[derive(Clone, Copy, Debug)]
pub enum TF_Graph {}

extern {
    pub fn TF_NewGraph() -> *mut TF_Graph;
    pub fn TF_DeleteGraph(graph: *mut TF_Graph);
}

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
    pub fn TF_NodeGetControlInputs(node: *mut TF_Node, inputs: *mut *mut TF_Node,
                                   max_inputs: c_int) -> c_int;
    pub fn TF_NodeNumControlOutputs(node: *mut TF_Node) -> c_int;
    pub fn TF_NodeGetControlOutputs(node: *mut TF_Node, outputs: *mut *mut TF_Node,
                                    max_outputs: c_int) -> c_int;
    pub fn TF_NodeGetAttrValueProto(node: *mut TF_Node, name: *const c_char, value: *mut TF_Buffer,
                                    status: *mut TF_Status);
    pub fn TF_GraphNodeByName(graph: *mut TF_Graph, name: *const c_char) -> *mut TF_Node;
    pub fn TF_GraphNextNode(graph: *mut TF_Graph, position: *mut size_t) -> *mut TF_Node;
    pub fn TF_GraphToGraphDef(graph: *mut TF_Graph, definition: *mut TF_Buffer,
                              status: *mut TF_Status);
    pub fn TF_NodeToNodeDef(node: *mut TF_Node, definition: *mut TF_Buffer,
                            status: *mut TF_Status);
}

#[derive(Clone, Copy, Debug)]
pub enum TF_SessionWithGraph {}

extern {
    pub fn TF_NewSessionWithGraph(graph: *mut TF_Graph, options: *const TF_SessionOptions,
                                  status: *mut TF_Status) -> *mut TF_SessionWithGraph;
    pub fn TF_CloseSessionWithGraph(session: *mut TF_SessionWithGraph, status: *mut TF_Status);
    pub fn TF_DeleteSessionWithGraph(session: *mut TF_SessionWithGraph, status: *mut TF_Status);
    pub fn TF_SessionRun(session: *mut TF_SessionWithGraph,
                         run_options: *const TF_Buffer,
                         inputs: *const TF_Port,
                         input_values: *const *mut TF_Tensor,
                         ninputs: c_int,
                         outputs: *const TF_Port,
                         output_values: *mut *mut TF_Tensor,
                         noutputs: c_int,
                         targets: *const *const TF_Node,
                         ntargets: c_int,
                         run_metadata: *mut TF_Buffer,
                         status: *mut TF_Status);
    pub fn TF_SessionPRunSetup(session: *mut TF_SessionWithGraph,
                               inputs: *const TF_Port,
                               ninputs: c_int,
                               outputs: *const TF_Port,
                               noutputs: c_int,
                               targets: *const *const TF_Node,
                               ntargets: c_int,
                               handle: *mut *const c_char,
                               status: *mut TF_Status);
    pub fn TF_SessionPRun(session: *mut TF_SessionWithGraph,
                          handle: *const c_char,
                          inputs: *const TF_Port,
                          input_values: *const *mut TF_Tensor,
                          ninputs: c_int,
                          outputs: *const TF_Port,
                          output_values: *mut *mut TF_Tensor,
                          noutputs: c_int,
                          targets: *const *const TF_Node,
                          ntargets: c_int,
                          status: *mut TF_Status);
}

#[derive(Clone, Copy, Debug)]
pub enum TF_Session {}

extern {
    pub fn TF_NewSession(options: *const TF_SessionOptions, status: *mut TF_Status)
                         -> *mut TF_Session;
    pub fn TF_CloseSession(session: *mut TF_Session, status: *mut TF_Status);
    pub fn TF_DeleteSession(session: *mut TF_Session, status: *mut TF_Status);
    pub fn TF_Reset(options: *const TF_SessionOptions, containers: *mut *const c_char,
                    ncontainers: c_int, status: *mut TF_Status);
    pub fn TF_ExtendGraph(session: *mut TF_Session, proto: *const c_void, proto_length: size_t,
                          status: *mut TF_Status);
    pub fn TF_Run(session: *mut TF_Session,
                  run_options: *const TF_Buffer,
                  inputs: *mut *const c_char,
                  input_values: *mut *mut TF_Tensor,
                  ninputs: c_int,
                  outputs: *mut *const c_char,
                  output_values: *mut *mut TF_Tensor,
                  noutputs: c_int,
                  targets: *mut *const c_char,
                  ntargets: c_int,
                  run_metadata: *mut TF_Buffer,
                  status: *mut TF_Status);
    pub fn TF_PRunSetup(session: *mut TF_Session,
                        inputs: *mut *const c_char,
                        ninputs: c_int,
                        outputs: *mut *const c_char,
                        noutputs: c_int,
                        targets: *mut *const c_char,
                        ntargets: c_int,
                        handle: *mut *const c_char,
                        status: *mut TF_Status);
    pub fn TF_PRun(session: *mut TF_Session,
                   handle: *const c_char,
                   inputs: *mut *const c_char,
                   input_values: *mut *mut TF_Tensor,
                   ninputs: c_int,
                   outputs: *mut *const c_char,
                   output_values: *mut *mut TF_Tensor,
                   noutputs: c_int,
                   targets: *mut *const c_char,
                   ntargets: c_int,
                   status: *mut TF_Status);
}

#[derive(Clone, Copy, Debug)]
pub enum TF_Library {}

extern {
    pub fn TF_LoadLibrary(name: *const c_char, status: *mut TF_Status) -> *mut TF_Library;
    pub fn TF_GetOpList(library: *mut TF_Library) -> TF_Buffer;
}
