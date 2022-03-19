macro_rules! stub {
    ($name:ident) => {
        #[no_mangle]
        pub extern "C" fn $name() {
            ()
        }
    };
}

//stub!(pw_context_connect);
//stub!(pw_context_connect_fd);
stub!(pw_context_destroy);
//stub!(pw_context_new);
//stub!(pw_core_disconnect);
stub!(pw_init);
stub!(pw_stream_add_listener);
stub!(pw_stream_connect);
stub!(pw_stream_dequeue_buffer);
stub!(pw_stream_destroy);
stub!(pw_stream_new);
stub!(pw_stream_queue_buffer);
stub!(pw_stream_update_params);
stub!(pw_thread_loop_destroy);
stub!(pw_thread_loop_get_loop);
stub!(pw_thread_loop_new);
stub!(pw_thread_loop_start);
stub!(pw_thread_loop_stop);

static CONTEXT: () = ();
static CORE: () = ();

#[no_mangle]
pub extern "C" fn pw_context_new(
    main_loop: *const (),
    properties: *const (),
    user_data_size: usize,
) -> *const () {
    &CONTEXT as *const ()
}

#[no_mangle]
pub extern "C" fn pw_context_connect(
    context: *const (),
    properties: *const (),
    user_data_size: usize,
) -> *const () {
    &CORE as *const ()
}

#[no_mangle]
pub extern "C" fn pw_context_connect_fd(
    context: *const (),
    fd: i32,
    properties: *const (),
    user_data_size: usize,
) -> *const () {
    &CORE as *const ()
}

#[no_mangle]
pub extern "C" fn pw_core_disconnect(core: *const ()) -> i32 {
    0
}
