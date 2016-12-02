use libc::{c_char, c_int, c_void, size_t};

use {TF_Buffer, TF_Graph, TF_Operation, TF_Output, TF_Status, TF_Tensor};

#[derive(Clone, Copy, Debug)]
pub enum TF_Session {}

#[derive(Clone, Copy, Debug)]
pub enum TF_SessionOptions {}

extern {
    pub fn TF_NewSession(graph: *mut TF_Graph,
                         options: *const TF_SessionOptions,
                         status: *mut TF_Status)
                         -> *mut TF_Session;
    pub fn TF_CloseSession(session: *mut TF_Session, status: *mut TF_Status);
    pub fn TF_DeleteSession(session: *mut TF_Session, status: *mut TF_Status);
    pub fn TF_SessionRun(session: *mut TF_Session,
                         run_options: *const TF_Buffer,
                         inputs: *const TF_Output,
                         input_values: *const *mut TF_Tensor,
                         num_inputs: c_int,
                         outputs: *const TF_Output,
                         output_values: *mut *mut TF_Tensor,
                         num_outputs: c_int,
                         targets: *const *const TF_Operation,
                         num_targets: c_int,
                         run_metadata: *mut TF_Buffer,
                         status: *mut TF_Status);
    pub fn TF_SessionPRunSetup(session: *mut TF_Session,
                               inputs: *const TF_Output,
                               num_inputs: c_int,
                               outputs: *const TF_Output,
                               num_outputs: c_int,
                               targets: *const *const TF_Operation,
                               num_targets: c_int,
                               handle: *mut *const c_char,
                               status: *mut TF_Status);
    pub fn TF_SessionPRun(session: *mut TF_Session,
                          handle: *const c_char,
                          inputs: *const TF_Output,
                          input_values: *const *mut TF_Tensor,
                          num_inputs: c_int,
                          outputs: *const TF_Output,
                          output_values: *mut *mut TF_Tensor,
                          num_outputs: c_int,
                          targets: *const *const TF_Operation,
                          num_targets: c_int,
                          status: *mut TF_Status);
    pub fn TF_Reset(options: *const TF_SessionOptions,
                    containers: *mut *const c_char,
                    num_containers: c_int,
                    status: *mut TF_Status);
}

extern {
    pub fn TF_NewSessionOptions() -> *mut TF_SessionOptions;
    pub fn TF_DeleteSessionOptions(options: *mut TF_SessionOptions);
    pub fn TF_SetTarget(options: *mut TF_SessionOptions, target: *const c_char);
    pub fn TF_SetConfig(options: *mut TF_SessionOptions,
                        proto: *const c_void,
                        proto_length: size_t,
                        status: *mut TF_Status);
}
