use libc::{c_char, c_int, c_void, size_t};

use {TF_Buffer, TF_Graph, TF_Node, TF_Port, TF_Status, TF_Tensor};

#[derive(Clone, Copy, Debug)]
pub enum TF_Session {}

#[derive(Clone, Copy, Debug)]
pub enum TF_SessionOptions {}

#[derive(Clone, Copy, Debug)]
pub enum TF_SessionWithGraph {}

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

extern {
    pub fn TF_NewSessionOptions() -> *mut TF_SessionOptions;
    pub fn TF_DeleteSessionOptions(options: *mut TF_SessionOptions);
    pub fn TF_SetTarget(options: *mut TF_SessionOptions, target: *const c_char);
    pub fn TF_SetConfig(options: *mut TF_SessionOptions, proto: *const c_void,
                        proto_length: size_t, status: *mut TF_Status);
}

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
