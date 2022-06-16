#[repr(C)]
pub struct CallbackBase {
    vtable: *const u8,
    flags: u8,
    callback: i32,
}

#[repr(C)]
pub struct CallResult<T, P> {
    base: Callback,
    api_call: Unk,
    object: *mut T,
    function: Option<unsafe extern "C" fn(arg1: *mut P, arg2: bool)>,
    _owns_t: PhantomData<T>,
    _owns_p: PhantomData<P>,
}
