#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    improper_ctypes,
    improper_ctypes_definitions,
    unnecessary_transmutes,
    clippy::all
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use libc::{c_int, c_ulong};

pub const PyUnicode_1BYTE_KIND: c_int = PyUnicode_Kind_PyUnicode_1BYTE_KIND as c_int;
pub const PyUnicode_2BYTE_KIND: c_int = PyUnicode_Kind_PyUnicode_2BYTE_KIND as c_int;
pub const PyUnicode_4BYTE_KIND: c_int = PyUnicode_Kind_PyUnicode_4BYTE_KIND as c_int;

pub const PyObject_HEAD_INIT: PyObject = PyObject {
    __bindgen_anon_1: _object__bindgen_ty_1 { ob_refcnt_full: 1 },
    ob_type: core::ptr::null_mut(),
};

pub const PyModuleDef_HEAD_INIT: PyModuleDef_Base = PyModuleDef_Base {
    ob_base: PyObject_HEAD_INIT,
    m_init: None,
    m_index: 0,
    m_copy: core::ptr::null_mut(),
};

pub const PyModuleDef_INIT: PyModuleDef = PyModuleDef {
    m_base: PyModuleDef_HEAD_INIT,
    m_name: core::ptr::null(),
    m_doc: core::ptr::null(),
    m_size: 0,
    m_methods: core::ptr::null_mut(),
    m_slots: core::ptr::null_mut(),
    m_traverse: None,
    m_clear: None,
    m_free: None,
};

pub const PyNumberMethods_INIT: PyNumberMethods = unsafe { core::mem::zeroed() };
pub const PySequenceMethods_INIT: PySequenceMethods = unsafe { core::mem::zeroed() };
pub const PyMappingMethods_INIT: PyMappingMethods = unsafe { core::mem::zeroed() };
pub const PyTypeObject_INIT: PyTypeObject = unsafe { core::mem::zeroed() };

impl PyTypeObject {
    #[inline]
    pub fn init_ob_type(&mut self, type_object: *mut PyTypeObject) {
        self.ob_base.ob_base.ob_type = type_object;
    }
}

#[inline(always)]
unsafe fn py_type_fast_subclass(tp: *mut PyTypeObject, flag: c_ulong) -> c_int {
    (((*tp).tp_flags & flag) != 0) as c_int
}

#[inline(always)]
pub unsafe fn Py_SIZE(ob: *mut PyObject) -> Py_ssize_t {
    (*(ob as *mut PyVarObject)).ob_size
}

#[inline(always)]
pub unsafe fn PyObject_TypeCheck(ob: *mut PyObject, tp: *mut PyTypeObject) -> c_int {
    (Py_TYPE(ob) == tp || PyType_IsSubtype(Py_TYPE(ob), tp) != 0) as c_int
}

#[inline(always)]
pub unsafe fn PyType_Check(op: *mut PyObject) -> c_int {
    py_type_fast_subclass(Py_TYPE(op), Py_TPFLAGS_TYPE_SUBCLASS as c_ulong)
}

#[inline(always)]
pub unsafe fn PyType_CheckExact(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == core::ptr::addr_of_mut!(PyType_Type)) as c_int
}

#[inline(always)]
pub unsafe fn PyType_HasFeature(t: *mut PyTypeObject, f: c_ulong) -> c_int {
    (((*t).tp_flags & f) != 0) as c_int
}

#[inline(always)]
pub unsafe fn PyType_IS_GC(t: *mut PyTypeObject) -> c_int {
    PyType_HasFeature(t, Py_TPFLAGS_HAVE_GC as c_ulong)
}

#[inline(always)]
pub unsafe fn Py_INCREF(op: *mut PyObject) {
    Py_IncRef(op)
}

#[inline(always)]
pub unsafe fn Py_DECREF(op: *mut PyObject) {
    Py_DecRef(op)
}

#[inline(always)]
pub unsafe fn Py_None() -> *mut PyObject {
    core::ptr::addr_of_mut!(_Py_NoneStruct)
}

#[inline(always)]
pub unsafe fn Py_NotImplemented() -> *mut PyObject {
    core::ptr::addr_of_mut!(_Py_NotImplementedStruct)
}

#[inline(always)]
pub unsafe fn PyBool_Check(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == core::ptr::addr_of_mut!(PyBool_Type)) as c_int
}

#[inline(always)]
pub unsafe fn Py_False() -> *mut PyObject {
    core::ptr::addr_of_mut!(_Py_FalseStruct) as *mut PyLongObject as *mut PyObject
}

#[inline(always)]
pub unsafe fn Py_True() -> *mut PyObject {
    core::ptr::addr_of_mut!(_Py_TrueStruct) as *mut PyLongObject as *mut PyObject
}

#[inline(always)]
pub unsafe fn PyDict_Check(op: *mut PyObject) -> c_int {
    py_type_fast_subclass(Py_TYPE(op), Py_TPFLAGS_DICT_SUBCLASS as c_ulong)
}

#[inline(always)]
pub unsafe fn PyList_Check(op: *mut PyObject) -> c_int {
    py_type_fast_subclass(Py_TYPE(op), Py_TPFLAGS_LIST_SUBCLASS as c_ulong)
}

#[inline(always)]
pub unsafe fn PyLong_Check(op: *mut PyObject) -> c_int {
    py_type_fast_subclass(Py_TYPE(op), Py_TPFLAGS_LONG_SUBCLASS as c_ulong)
}

#[inline(always)]
pub unsafe fn PyFloat_Check(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, core::ptr::addr_of_mut!(PyFloat_Type))
}

#[inline(always)]
pub unsafe fn PyModule_Check(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, core::ptr::addr_of_mut!(PyModule_Type))
}

#[inline(always)]
pub unsafe fn PyTuple_Check(op: *mut PyObject) -> c_int {
    py_type_fast_subclass(Py_TYPE(op), Py_TPFLAGS_TUPLE_SUBCLASS as c_ulong)
}

#[inline(always)]
pub unsafe fn PyTuple_GET_ITEM(op: *mut PyObject, i: Py_ssize_t) -> *mut PyObject {
    PyTuple_GetItem(op, i)
}

#[inline(always)]
pub unsafe fn PyTuple_GET_SIZE(op: *mut PyObject) -> Py_ssize_t {
    PyTuple_Size(op)
}

#[inline(always)]
pub unsafe fn PyBytes_Check(op: *mut PyObject) -> c_int {
    py_type_fast_subclass(Py_TYPE(op), Py_TPFLAGS_BYTES_SUBCLASS as c_ulong)
}

#[inline(always)]
pub unsafe fn PyUnicode_Check(op: *mut PyObject) -> c_int {
    py_type_fast_subclass(Py_TYPE(op), Py_TPFLAGS_UNICODE_SUBCLASS as c_ulong)
}

#[inline(always)]
pub unsafe fn PyUnicode_READY(_op: *mut PyObject) -> c_int {
    0
}

#[inline(always)]
pub unsafe fn PyUnicode_GET_LENGTH(op: *mut PyObject) -> Py_ssize_t {
    PyUnicode_GetLength(op)
}

#[inline(always)]
pub unsafe fn PyCapsule_CheckExact(ob: *mut PyObject) -> c_int {
    (Py_TYPE(ob) == core::ptr::addr_of_mut!(PyCapsule_Type)) as c_int
}

#[inline(always)]
pub unsafe fn PySet_CheckExact(ob: *mut PyObject) -> c_int {
    (Py_TYPE(ob) == core::ptr::addr_of_mut!(PySet_Type)) as c_int
}

#[inline(always)]
pub unsafe fn PySet_Check(ob: *mut PyObject) -> c_int {
    (PySet_CheckExact(ob) != 0
        || PyType_IsSubtype(Py_TYPE(ob), core::ptr::addr_of_mut!(PySet_Type)) != 0)
        as c_int
}

#[inline(always)]
pub unsafe fn PyExceptionClass_Check(x: *mut PyObject) -> c_int {
    (PyType_Check(x) != 0
        && py_type_fast_subclass(x as *mut PyTypeObject, Py_TPFLAGS_BASE_EXC_SUBCLASS as c_ulong)
            != 0) as c_int
}

#[inline(always)]
pub unsafe fn PyExceptionInstance_Check(x: *mut PyObject) -> c_int {
    py_type_fast_subclass(Py_TYPE(x), Py_TPFLAGS_BASE_EXC_SUBCLASS as c_ulong)
}

#[inline(always)]
pub unsafe fn PyExceptionInstance_Class(x: *mut PyObject) -> *mut PyObject {
    Py_TYPE(x) as *mut PyObject
}

#[inline(always)]
pub unsafe fn PyModule_Create(module: *mut PyModuleDef) -> *mut PyObject {
    PyModule_Create2(module, PYTHON_API_VERSION as c_int)
}

#[inline(always)]
pub unsafe fn PyEval_ThreadsInitialized() -> c_int {
    Py_IsInitialized()
}
