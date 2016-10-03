use libc::{c_char, c_float, c_int, c_uchar, c_void, int64_t, size_t};

use {TF_Buffer, TF_DataType, TF_Status, TF_Tensor};

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TF_AttrType {
    TF_ATTR_STRING = 0,
    TF_ATTR_INT = 1,
    TF_ATTR_FLOAT = 2,
    TF_ATTR_BOOL = 3,
    TF_ATTR_TYPE = 4,
    TF_ATTR_SHAPE = 5,
    TF_ATTR_TENSOR = 6,
    TF_ATTR_PLACEHOLDER = 7,
    TF_ATTR_FUNC = 8,
}
pub use TF_AttrType::*;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TF_AttrMetadata {
    pub is_list: c_uchar,
    pub list_size: int64_t,
    pub kind: TF_AttrType,
    pub total_size: int64_t,
}

#[derive(Clone, Copy, Debug)]
pub enum TF_Graph {}

#[derive(Clone, Copy, Debug)]
pub enum TF_ImportGraphDefOptions {}

#[derive(Clone, Copy, Debug)]
pub enum TF_Operation {}

#[derive(Clone, Copy, Debug)]
pub enum TF_OperationDescription {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TF_Port {
    pub operation: *mut TF_Operation,
    pub index: c_int,
}

extern {
    pub fn TF_NewGraph() -> *mut TF_Graph;
    pub fn TF_DeleteGraph(graph: *mut TF_Graph);
    pub fn TF_GraphSetTensorShape(graph: *mut TF_Graph,
                                  port: TF_Port,
                                  dims: *const int64_t,
                                  num_dims: c_int,
                                  status: *mut TF_Status);
    pub fn TF_GraphGetTensorNumDims(graph: *mut TF_Graph,
                                    port: TF_Port,
                                    status: *mut TF_Status)
                                    -> c_int;
    pub fn TF_GraphGetTensorShape(graph: *mut TF_Graph,
                                  port: TF_Port,
                                  dims: *mut int64_t,
                                  num_dims: c_int,
                                  status: *mut TF_Status);
    pub fn TF_GraphOperationByName(graph: *mut TF_Graph, name: *const c_char) -> *mut TF_Operation;
    pub fn TF_GraphNextOperation(graph: *mut TF_Graph, position: *mut size_t) -> *mut TF_Operation;
    pub fn TF_GraphToGraphDef(graph: *mut TF_Graph,
                              definition: *mut TF_Buffer,
                              status: *mut TF_Status);
    pub fn TF_GraphImportGraphDef(graph: *mut TF_Graph,
                                  definition: *const TF_Buffer,
                                  options: *const TF_ImportGraphDefOptions,
                                  status: *mut TF_Status);
}

extern {
    pub fn TF_NewImportGraphDefOptions() -> *mut TF_ImportGraphDefOptions;
    pub fn TF_DeleteImportGraphDefOptions(options: *mut TF_ImportGraphDefOptions);
    pub fn TF_ImportGraphDefOptionsSetPrefix(options: *mut TF_ImportGraphDefOptions,
                                             prefix: *const c_char);
}

extern {
    pub fn TF_OperationName(operation: *mut TF_Operation) -> *const c_char;
    pub fn TF_OperationOpType(operation: *mut TF_Operation) -> *const c_char;
    pub fn TF_OperationDevice(operation: *mut TF_Operation) -> *const c_char;
    pub fn TF_OperationNumOutputs(operation: *mut TF_Operation) -> c_int;
    pub fn TF_OperationOutputType(output: TF_Port) -> TF_DataType;
    pub fn TF_OperationOutputListLength(operation: *mut TF_Operation,
                                        name: *const c_char,
                                        status: *mut TF_Status)
                                        -> c_int;
    pub fn TF_OperationNumInputs(operation: *mut TF_Operation) -> c_int;
    pub fn TF_OperationInputType(input: TF_Port) -> TF_DataType;
    pub fn TF_OperationInputListLength(operation: *mut TF_Operation,
                                       name: *const c_char,
                                       status: *mut TF_Status)
                                       -> c_int;
    pub fn TF_OperationInput(input: TF_Port) -> TF_Port;
    pub fn TF_OperationOutputNumConsumers(output: TF_Port) -> c_int;
    pub fn TF_OperationOutputConsumers(output: TF_Port,
                                       consumers: *mut TF_Port,
                                       max_consumers: c_int)
                                       -> c_int;
    pub fn TF_OperationNumControlInputs(operation: *mut TF_Operation) -> c_int;
    pub fn TF_OperationGetControlInputs(operation: *mut TF_Operation,
                                        control_inputs: *mut *mut TF_Operation,
                                        max_control_inputs: c_int)
                                        -> c_int;
    pub fn TF_OperationNumControlOutputs(operation: *mut TF_Operation) -> c_int;
    pub fn TF_OperationGetControlOutputs(operation: *mut TF_Operation,
                                         control_outputs: *mut *mut TF_Operation,
                                         max_control_outputs: c_int)
                                         -> c_int;
    pub fn TF_OperationGetAttrMetadata(operation: *mut TF_Operation,
                                       name: *const c_char,
                                       status: *mut TF_Status)
                                       -> TF_AttrMetadata;
    pub fn TF_OperationGetAttrString(operation: *mut TF_Operation,
                                     name: *const c_char,
                                     value: *mut c_void,
                                     max_length: c_int,
                                     status: *mut TF_Status);
    pub fn TF_OperationGetAttrStringList(operation: *mut TF_Operation,
                                         name: *const c_char,
                                         values: *mut *mut c_void,
                                         lengths: *mut c_int,
                                         max_values: c_int,
                                         storage: *mut c_void,
                                         storage_size: size_t,
                                         status: *mut TF_Status);
    pub fn TF_OperationGetAttrInt(operation: *mut TF_Operation,
                                  name: *const c_char,
                                  value: *mut int64_t,
                                  status: *mut TF_Status);
    pub fn TF_OperationGetAttrIntList(operation: *mut TF_Operation,
                                      name: *const c_char,
                                      values: *mut int64_t,
                                      max_values: c_int,
                                      status: *mut TF_Status);
    pub fn TF_OperationGetAttrFloat(operation: *mut TF_Operation,
                                    name: *const c_char,
                                    value: *mut c_float,
                                    status: *mut TF_Status);
    pub fn TF_OperationGetAttrFloatList(operation: *mut TF_Operation,
                                        name: *const c_char,
                                        values: *mut c_float,
                                        max_values: c_int,
                                        status: *mut TF_Status);
    pub fn TF_OperationGetAttrBool(operation: *mut TF_Operation,
                                   name: *const c_char,
                                   value: *mut c_uchar,
                                   status: *mut TF_Status);
    pub fn TF_OperationGetAttrBoolList(operation: *mut TF_Operation,
                                       name: *const c_char,
                                       values: *mut c_char,
                                       max_values: c_int,
                                       status: *mut TF_Status);
    pub fn TF_OperationGetAttrType(operation: *mut TF_Operation,
                                   name: *const c_char,
                                   value: *mut TF_DataType,
                                   status: *mut TF_Status);
    pub fn TF_OperationGetAttrTypeList(operation: *mut TF_Operation,
                                       name: *const c_char,
                                       values: *mut TF_DataType,
                                       max_values: c_int,
                                       status: *mut TF_Status);
    pub fn TF_OperationGetAttrShape(operation: *mut TF_Operation,
                                    name: *const c_char,
                                    value: *mut int64_t,
                                    num_dims: c_int,
                                    status: *mut TF_Status);
    pub fn TF_OperationGetAttrShapeList(operation: *mut TF_Operation,
                                        name: *const c_char,
                                        dims: *mut *mut int64_t,
                                        num_dims: *mut c_int,
                                        num_shapes: c_int,
                                        storage: *mut int64_t,
                                        storage_size: c_int,
                                        status: *mut TF_Status);
    pub fn TF_OperationGetAttrTensorShapeProto(operation: *mut TF_Operation,
                                               name: *const c_char,
                                               value: *mut TF_Buffer,
                                               status: *mut TF_Status);
    pub fn TF_OperationGetAttrTensorShapeProtoList(operation: *mut TF_Operation,
                                                   name: *const c_char,
                                                   values: *mut *mut TF_Buffer,
                                                   max_values: c_int,
                                                   status: *mut TF_Status);
    pub fn TF_OperationGetAttrTensor(operation: *mut TF_Operation,
                                     name: *const c_char,
                                     value: *mut *mut TF_Tensor,
                                     status: *mut TF_Status);
    pub fn TF_OperationGetAttrTensorList(operation: *mut TF_Operation,
                                         name: *const c_char,
                                         values: *mut TF_Tensor,
                                         max_values: c_int,
                                         status: *mut TF_Status);
    pub fn TF_OperationGetAttrValueProto(operation: *mut TF_Operation,
                                         name: *const c_char,
                                         value: *mut TF_Buffer,
                                         status: *mut TF_Status);
    pub fn TF_OperationToNodeDef(operation: *mut TF_Operation,
                                 definition: *mut TF_Buffer,
                                 status: *mut TF_Status);
}

extern {
    pub fn TF_NewOperation(graph: *mut TF_Graph,
                           operation_type: *const c_char,
                           name: *const c_char)
                           -> *mut TF_OperationDescription;
    pub fn TF_SetDevice(description: *mut TF_OperationDescription, device: *const c_char);
    pub fn TF_AddInput(description: *mut TF_OperationDescription, input: TF_Port);
    pub fn TF_AddInputList(description: *mut TF_OperationDescription,
                           inputs: *const TF_Port,
                           num_inputs: c_int);
    pub fn TF_AddControlInput(description: *mut TF_OperationDescription,
                              operation: *mut TF_Operation);
    pub fn TF_ColocateWith(description: *mut TF_OperationDescription,
                           operation: *mut TF_Operation);
    pub fn TF_SetAttrString(description: *mut TF_OperationDescription,
                            name: *const c_char,
                            value: *const c_void,
                            length: c_int);
    pub fn TF_SetAttrStringList(description: *mut TF_OperationDescription,
                                name: *const c_char,
                                values: *const *const c_void,
                                lengths: *const c_int,
                                num_values: c_int);
    pub fn TF_SetAttrInt(description: *mut TF_OperationDescription,
                         name: *const c_char,
                         value: int64_t);
    pub fn TF_SetAttrIntList(description: *mut TF_OperationDescription,
                             name: *const c_char,
                             values: *const int64_t,
                             num_values: c_int);
    pub fn TF_SetAttrFloat(description: *mut TF_OperationDescription,
                           name: *const c_char,
                           value: c_float);
    pub fn TF_SetAttrFloatList(description: *mut TF_OperationDescription,
                               name: *const c_char,
                               values: *const c_float,
                               num_values: c_int);
    pub fn TF_SetAttrBool(description: *mut TF_OperationDescription,
                          name: *const c_char,
                          value: c_uchar);
    pub fn TF_SetAttrBoolList(description: *mut TF_OperationDescription,
                              name: *const c_char,
                              values: *const c_uchar,
                              num_values: c_int);
    pub fn TF_SetAttrType(description: *mut TF_OperationDescription,
                          name: *const c_char,
                          value: TF_DataType);
    pub fn TF_SetAttrTypeList(description: *mut TF_OperationDescription,
                              name: *const c_char,
                              values: *const TF_DataType,
                              num_values: c_int);
    pub fn TF_SetAttrShape(description: *mut TF_OperationDescription,
                           name: *const c_char,
                           dims: *const int64_t,
                           num_dims: c_int);
    pub fn TF_SetAttrShapeList(description: *mut TF_OperationDescription,
                               name: *const c_char,
                               dims: *const *const int64_t,
                               num_dims: *const c_int,
                               num_shapes: c_int);
    pub fn TF_SetAttrTensorShapeProto(description: *mut TF_OperationDescription,
                                      name: *const c_char,
                                      proto: *const c_void,
                                      proto_length: c_int,
                                      status: *mut TF_Status);
    pub fn TF_SetAttrTensorShapeProtoList(description: *mut TF_OperationDescription,
                                          name: *const c_char,
                                          protos: *const *const c_void,
                                          proto_lengths: *const c_int,
                                          num_shapes: c_int,
                                          status: *mut TF_Status);
    pub fn TF_SetAttrTensor(description: *mut TF_OperationDescription,
                            name: *const c_char,
                            value: *mut TF_Tensor,
                            status: *mut TF_Status);
    pub fn TF_SetAttrTensorList(description: *mut TF_OperationDescription,
                                name: *const c_char,
                                values: *const *mut TF_Tensor,
                                num_values: c_int,
                                status: *mut TF_Status);
    pub fn TF_SetAttrValueProto(description: *mut TF_OperationDescription,
                                name: *const c_char,
                                proto: *const c_void,
                                proto_length: size_t,
                                status: *mut TF_Status);
    pub fn TF_FinishOperation(description: *mut TF_OperationDescription,
                              status: *mut TF_Status)
                              -> *mut TF_Operation;
}
