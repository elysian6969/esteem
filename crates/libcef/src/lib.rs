#![allow(unused_variables)]

macro_rules! stub {
    ($name:ident) => {
        #[no_mangle]
        pub extern "C" fn $name() {
            frosting::println!();

            ()
        }
    };
}

// ty cef
// https://github.com/chromiumembedded/cef/blob/master/include/cef_api_hash.h
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Entry {
    // CEF_API_HASH_PLATFORM
    Platform = 0,
    // CEF_API_HASH_UNIVERSAL
    Universal = 1,
    // CEF_COMMIT_HASH
    Commit = 2,
}

#[no_mangle]
pub extern "C" fn cef_api_hash(entry: Entry) -> *const u8 {
    frosting::println!("entry = {:?}", entry);

    "d686964de5e01eed900d78d5f8a7da9f8fc3a777".as_ptr()
}

stub!(cef_binary_value_create);
stub!(cef_browser_host_create_browser);
stub!(cef_command_line_create);
stub!(cef_cookie_manager_get_global_manager);
stub!(cef_crash_reporting_enabled);
stub!(cef_currently_on);
stub!(cef_do_message_loop_work);
stub!(cef_enable_highdpi_support);
stub!(cef_execute_process);
stub!(cef_get_xdisplay);
stub!(cef_initialize);
stub!(cef_list_value_create);
stub!(cef_parse_url);
stub!(cef_post_data_create);
stub!(cef_post_data_element_create);
stub!(cef_post_delayed_task);
stub!(cef_post_task);
stub!(cef_process_message_create);
stub!(cef_quit_message_loop);
stub!(cef_register_widevine_cdm);
stub!(cef_request_create);
stub!(cef_run_message_loop);
stub!(cef_set_crash_key_value);
stub!(cef_shutdown);
stub!(cef_stream_reader_create_for_data);
stub!(cef_stream_reader_create_for_file);
stub!(cef_string_list_alloc);
stub!(cef_string_list_append);
stub!(cef_string_list_free);
stub!(cef_string_list_size);
stub!(cef_string_list_value);
stub!(cef_string_map_alloc);
stub!(cef_string_map_append);
stub!(cef_string_map_free);
stub!(cef_string_map_key);
stub!(cef_string_map_size);
stub!(cef_string_map_value);
stub!(cef_string_multimap_alloc);
stub!(cef_string_multimap_append);
stub!(cef_string_multimap_free);
stub!(cef_string_multimap_key);
stub!(cef_string_multimap_size);
stub!(cef_string_multimap_value);
stub!(cef_string_userfree_utf8_free);
stub!(cef_string_utf8_clear);
stub!(cef_string_utf8_cmp);
stub!(cef_string_utf8_set);
stub!(cef_string_wide_to_utf8);
stub!(cef_time_to_timet);
stub!(cef_urlrequest_create);
stub!(cef_v8context_get_current_context);
stub!(cef_v8value_create_array);
stub!(cef_v8value_create_array_buffer);
stub!(cef_v8value_create_bool);
stub!(cef_v8value_create_double);
stub!(cef_v8value_create_function);
stub!(cef_v8value_create_int);
stub!(cef_v8value_create_null);
stub!(cef_v8value_create_object);
stub!(cef_v8value_create_string);
stub!(cef_v8value_create_uint);
