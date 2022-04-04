#[macro_export]
macro_rules! api_fn {
    (
        $name:ident(&$instance:ident) -> $output:ty {
            $($body:tt)*
        }
    ) => {
        paste::paste! {
            #[no_mangle]
            pub extern "C" fn [<SteamAPI_ISteam $instance _ $name>](this: *const $instance) -> $output {
                $($body)*
            }
        }
    };

    (
        $name:ident(&$instance:ident, $($arg:ident: $argty:ty),* $(,)*) -> $output:ty {
            $($body:tt)*
        }
    ) => {
        paste::paste! {
            #[no_mangle]
            pub extern "C" fn [<SteamAPI_ISteam $instance _ $name>](this: *const $instance, $($arg: $argty),*) -> $output {
                $($body)*
            }
        }
    };
}

#[macro_export]
macro_rules! debug {
    () => {{
        println!("{}", frosting::function!());
    }};
}

#[macro_export]
macro_rules! virtual_struct {
    // struct generation

    (
        @signature
        $instance:ident {}
        ->
        ($($result:tt)*)
    ) => {
        #[repr(C)]
        pub struct $instance {
            pub vtable: &'static VTable,
        }

        #[repr(C)]
        pub struct VTable {
            $($result)*
        }
    };

    // signature consumers

    (
        @signature
        $instance:ident {
            fn $ident:ident(&self) -> $output:ty,
            $($rest:tt)*
        }
        ->
        ($($result:tt)*)
    ) => (
        $crate::virtual_struct!(@signature $instance { $($rest)* } -> (
            $($result)*
            pub $ident: extern "C" fn(this: *const $instance) -> $output,
        ));
    );

    (
        @signature
        $instance:ident {
            fn $ident:ident(&self, $($arg:ident: $argty:ty),*) -> $output:ty,
            $($rest:tt)*
        }
        ->
        ($($result:tt)*)
    ) => (
        $crate::virtual_struct!(@signature $instance { $($rest)* } -> (
            $($result)*
            pub $ident: extern "C" fn(this: *const $instance, $($arg: $argty),*) -> $output,
        ));
    );

    // entry points

    (
        $instance:ident {}
    ) => (
        $crate::virtual_struct!(@signature $instance { } -> ());
    );

    (
        $instance:ident {
            fn $name:ident(&self) -> $output:ty,
            $($rest:tt)*
        }
    ) => (
        $crate::virtual_struct!(@signature $instance {
            fn $name(&self) -> $output,
            $($rest)*
        } -> ());
    );

    (
        $instance:ident {
            fn $name:ident(&self, $($arg:ident: $argty:ty),*) -> $output:ty,
            $($rest:tt)*
        }
    ) => (
        $crate::virtual_struct!(@signature $instance {
            fn $name(&self, $($arg: $argty),*) -> $output,
            $($rest)*
        } -> ());
    );
}
