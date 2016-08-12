use libc::{c_char, c_float, c_int, c_uchar, c_void, int64_t, size_t};

use {TF_Buffer, TF_DataType, TF_Status, TF_Tensor};

#[derive(Clone, Copy, Debug)]
pub enum TF_Graph {}

#[derive(Clone, Copy, Debug)]
pub enum TF_OperationDescription {}

#[derive(Clone, Copy, Debug)]
pub enum TF_Operation {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TF_Port {
    pub oper: *mut TF_Operation,
    pub index: c_int,
}

extern {
    pub fn TF_NewGraph() -> *mut TF_Graph;
    pub fn TF_DeleteGraph(graph: *mut TF_Graph);
    pub fn TF_GraphOperationByName(graph: *mut TF_Graph, name: *const c_char) -> *mut TF_Operation;
    pub fn TF_GraphNextOperation(graph: *mut TF_Graph, position: *mut size_t) -> *mut TF_Operation;
    pub fn TF_GraphToGraphDef(graph: *mut TF_Graph, definition: *mut TF_Buffer,
                              status: *mut TF_Status);
}

extern {
    pub fn TF_NewOperation(graph: *mut TF_Graph, operation_type: *const c_char,
                           name: *const c_char) -> *mut TF_OperationDescription;
    pub fn TF_SetDevice(description: *mut TF_OperationDescription, device: *const c_char);
    pub fn TF_AddInput(description: *mut TF_OperationDescription, input: TF_Port);
    pub fn TF_AddInputList(description: *mut TF_OperationDescription, inputs: *const TF_Port,
                           ninputs: c_int);
    pub fn TF_AddControlInput(description: *mut TF_OperationDescription, input: *mut TF_Operation);
    pub fn TF_SetAttrString(description: *mut TF_OperationDescription, name: *const c_char,
                            value: *const c_void, length: c_int);
    pub fn TF_SetAttrStringList(description: *mut TF_OperationDescription, name: *const c_char,
                                values: *const *const c_void, lengths: *const c_int,
                                nvalues: c_int);
    pub fn TF_SetAttrInt(description: *mut TF_OperationDescription, name: *const c_char,
                         value: int64_t);
    pub fn TF_SetAttrIntList(description: *mut TF_OperationDescription, name: *const c_char,
                             values: *const int64_t, nvalues: c_int);
    pub fn TF_SetAttrFloat(description: *mut TF_OperationDescription, name: *const c_char,
                           value: c_float);
    pub fn TF_SetAttrFloatList(description: *mut TF_OperationDescription, name: *const c_char,
                               values: *const c_float, nvalues: c_int);
    pub fn TF_SetAttrBool(description: *mut TF_OperationDescription, name: *const c_char,
                          value: c_uchar);
    pub fn TF_SetAttrBoolList(description: *mut TF_OperationDescription, name: *const c_char,
                              values: *const c_uchar, nvalues: c_int);
    pub fn TF_SetAttrType(description: *mut TF_OperationDescription, name: *const c_char,
                          value: TF_DataType);
    pub fn TF_SetAttrTypeList(description: *mut TF_OperationDescription, name: *const c_char,
                              values: *const TF_DataType, nvalues: c_int);
    pub fn TF_SetAttrShape(description: *mut TF_OperationDescription, name: *const c_char,
                           dims: *const int64_t, ndims: c_int);
    pub fn TF_SetAttrShapeList(description: *mut TF_OperationDescription, name: *const c_char,
                               dims: *const *const int64_t, ndims: *const c_int, nshapes: c_int);
    pub fn TF_SetAttrTensorShapeProto(description: *mut TF_OperationDescription,
                                      name: *const c_char, proto: *mut c_void, proto_length: c_int,
                                      status: *mut TF_Status);
    pub fn TF_SetAttrTensorShapeProtoList(description: *mut TF_OperationDescription,
                                          name: *const c_char, protos: *const *const c_void,
                                          proto_lengths: *const c_int, nshapes: c_int,
                                          status: *mut TF_Status);
    pub fn TF_SetAttrTensor(description: *mut TF_OperationDescription, name: *const c_char,
                            value: *mut TF_Tensor, status: *mut TF_Status);
    pub fn TF_SetAttrTensorList(description: *mut TF_OperationDescription, name: *const c_char,
                                values: *const *mut TF_Tensor, nvalues: c_int,
                                status: *mut TF_Status);
    pub fn TF_SetAttrToAttrValueProto(description: *mut TF_OperationDescription,
                                      name: *const c_char, proto: *const c_void,
                                      proto_length: size_t, status: *mut TF_Status);
    pub fn TF_FinishOperation(description: *mut TF_OperationDescription, status: *mut TF_Status)
                              -> *mut TF_Operation;
}

extern {
    pub fn TF_OperationName(operation: *mut TF_Operation) -> *const c_char;
    pub fn TF_OperationOpType(operation: *mut TF_Operation) -> *const c_char;
    pub fn TF_OperationDevice(operation: *mut TF_Operation) -> *const c_char;
    pub fn TF_OperationNumOutputs(operation: *mut TF_Operation) -> c_int;
    pub fn TF_OperationOutputType(output: TF_Port) -> TF_DataType;
    pub fn TF_OperationOutputListLength(operation: *mut TF_Operation, name: *const c_char,
                                        status: *mut TF_Status) -> c_int;
    pub fn TF_OperationNumInputs(operation: *mut TF_Operation) -> c_int;
    pub fn TF_OperationInputType(input: TF_Port) -> TF_DataType;
    pub fn TF_OperationInputListLength(operation: *mut TF_Operation, name: *const c_char,
                                       status: *mut TF_Status) -> c_int;
    pub fn TF_OperationInput(input: TF_Port) -> TF_Port;
    pub fn TF_OperationOutputNumConsumers(output: TF_Port) -> c_int;
    pub fn TF_OperationOutputConsumers(output: TF_Port, consumers: *mut TF_Port,
                                       max_consumers: c_int) -> c_int;
    pub fn TF_OperationNumControlInputs(operation: *mut TF_Operation) -> c_int;
    pub fn TF_OperationGetControlInputs(operation: *mut TF_Operation,
                                        control_inputs: *mut *mut TF_Operation,
                                        max_control_inputs: c_int) -> c_int;
    pub fn TF_OperationNumControlOutputs(operation: *mut TF_Operation) -> c_int;
    pub fn TF_OperationGetControlOutputs(operation: *mut TF_Operation,
                                         control_outputs: *mut *mut TF_Operation,
                                         max_control_outputs: c_int) -> c_int;
    pub fn TF_OperationGetAttrValueProto(operation: *mut TF_Operation, name: *const c_char,
                                         value: *mut TF_Buffer, status: *mut TF_Status);
    pub fn TF_OperationToNodeDef(operation: *mut TF_Operation, definition: *mut TF_Buffer,
                                 status: *mut TF_Status);
}
