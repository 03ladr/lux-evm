#[derive(Debug)]
pub enum StatusCode {
    Completion,
    Failure,
    Revert,
    OutOfGas,
    InvalidInstruction,
    UndefinedInstruction,
    StackOverflow,
    StackUnderflow,
    BadJumpDest,
    InvalidMemoryAccess,
    CallDepthExceeded,
    StaticModeViolation,
    PrecompileFailure,
    ContractValidationFailure,
    ArgOutOfRange,
    WASMUnreachableInstruction,
    WASMTrap,
    InsufficientBalance,
    InternalError,
    Rejected,
    OutOfMemory,
}
