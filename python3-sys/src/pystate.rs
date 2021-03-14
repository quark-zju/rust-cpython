use libc::c_int;

#[cfg(Py_3_9)]
use crate::frameobject::PyFrameObject;
use crate::moduleobject::PyModuleDef;
use crate::object::PyObject;

#[cfg(Py_3_6)]
pub const MAX_CO_EXTRA_USERS: libc::c_int = 255;

#[repr(C)]
pub struct PyInterpreterState {
    _private: [u8; 0],
}

pub type Py_tracefunc = unsafe extern "C" fn(
    arg1: *mut PyObject,
    arg2: *mut PyFrameObject,
    arg3: c_int,
    arg4: *mut PyObject,
) -> c_int;

/* The following values are used for 'what' for tracefunc functions: */
pub const PyTrace_CALL: c_int = 0;
pub const PyTrace_EXCEPTION: c_int = 1;
pub const PyTrace_LINE: c_int = 2;
pub const PyTrace_RETURN: c_int = 3;
pub const PyTrace_C_CALL: c_int = 4;
pub const PyTrace_C_EXCEPTION: c_int = 5;
pub const PyTrace_C_RETURN: c_int = 6;
pub const PyTrace_OPCODE: c_int = 7;

#[repr(C)]
pub struct PyThreadState {
    _private: [u8; 0],
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyInterpreterState_New() -> *mut PyInterpreterState;
    pub fn PyInterpreterState_Clear(arg1: *mut PyInterpreterState) -> ();
    pub fn PyInterpreterState_Delete(arg1: *mut PyInterpreterState) -> ();
    #[cfg(Py_3_9)]
    pub fn PyInterpreterState_Get() -> *mut PyInterpreterState;
    #[cfg(Py_3_8)]
    pub fn PyInterpreterState_GetDict(arg1: *mut PyInterpreterState) -> *mut PyObject;
    #[cfg(Py_3_7)]
    pub fn PyInterpreterState_GetID(arg1: *mut PyInterpreterState) -> i64;
    pub fn PyState_FindModule(arg1: *mut PyModuleDef) -> *mut PyObject;
    pub fn PyThreadState_New(arg1: *mut PyInterpreterState) -> *mut PyThreadState;
    ignore! {
        fn _PyThreadState_Prealloc(arg1: *mut PyInterpreterState) -> *mut PyThreadState;
        fn _PyThreadState_Init(arg1: *mut PyThreadState) -> ();
    }
    pub fn PyThreadState_Clear(arg1: *mut PyThreadState) -> ();
    pub fn PyThreadState_Delete(arg1: *mut PyThreadState) -> ();
    #[cfg(any(Py_3_7, py_sys_config = "WITH_THREAD"))]
    pub fn PyThreadState_DeleteCurrent() -> ();
    pub fn PyThreadState_Get() -> *mut PyThreadState;
    pub fn PyThreadState_Swap(arg1: *mut PyThreadState) -> *mut PyThreadState;
    pub fn PyThreadState_GetDict() -> *mut PyObject;
    #[cfg(not(Py_3_7))]
    pub fn PyThreadState_SetAsyncExc(arg1: libc::c_long, arg2: *mut PyObject) -> libc::c_int;
    #[cfg(Py_3_7)]
    pub fn PyThreadState_SetAsyncExc(arg1: libc::c_ulong, arg2: *mut PyObject) -> libc::c_int;
    #[cfg(Py_3_9)]
    pub fn PyThreadState_GetInterpreter(tstate: *mut PyThreadState) -> *mut PyInterpreterState;
    #[cfg(Py_3_9)]
    pub fn PyThreadState_GetFrame(tstate: *mut PyThreadState) -> *mut PyFrameObject;
    #[cfg(Py_3_9)]
    pub fn PyThreadState_GetID(tstate: *mut PyThreadState) -> u64;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PyGILState_STATE {
    PyGILState_LOCKED,
    PyGILState_UNLOCKED,
}

#[cfg(any(Py_3_7, py_sys_config = "WITH_THREAD"))]
#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyGILState_Ensure() -> PyGILState_STATE;
    pub fn PyGILState_Release(arg1: PyGILState_STATE) -> ();
    pub fn PyGILState_GetThisThreadState() -> *mut PyThreadState;
}

#[inline(always)]
pub unsafe fn PyThreadState_GET() -> *mut PyThreadState {
    PyThreadState_Get()
}
