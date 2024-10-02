use ffi::SubObjectFromC;

mod ffi {
    use std::{
        ffi::{c_char, c_int, c_void, CStr},
        marker::PhantomData,
        ptr::NonNull,
    };

    extern "C" {
        fn init_context() -> *mut c_void;
        fn free_context(context: *mut c_void);

        fn context_is_lower(context: *const c_void) -> c_int;
        fn context_is_upper(context: *const c_void) -> c_int;
        fn context_to_lower(context: *mut c_void);
        fn context_to_upper(context: *mut c_void);

        fn create_subobject(
            context: *const c_void,
            offset: c_int,
            sub_obj: *mut *mut c_void,
        ) -> c_int;

        pub fn set_callback(f: extern "C" fn(*const c_char));
    }

    // implements RAII but has no safety measures
    //pub struct ContextFromC {
    //    inner: *mut c_void,
    //}

    #[non_exhaustive]
    #[repr(transparent)]
    pub struct ContextFromC(NonNull<c_void>);

    impl ContextFromC {
        pub fn new() -> Self {
            Self {
                0: unsafe { NonNull::new(init_context()).unwrap() },
            }
        }

        pub fn is_lower(&self) -> bool {
            let res = unsafe { context_is_lower(self.0.as_ptr()) };
            res == 1
        }

        pub fn is_upper(&self) -> bool {
            let res = unsafe { context_is_upper(self.0.as_ptr()) };
            res == 1
        }

        pub fn to_lower(&mut self) {
            unsafe {
                context_to_lower(self.0.as_mut());
            }
        }

        pub fn to_upper(&mut self) {
            unsafe {
                context_to_upper(self.0.as_mut());
            }
        }
    }

    impl Drop for ContextFromC {
        fn drop(&mut self) {
            unsafe {
                free_context(self.0.as_mut());
            }
        }
    }

    pub struct SubObjectFromC<'a> {
        inner: *const c_char,
        _covariant: PhantomData<&'a mut ()>,
    }
    impl<'a> SubObjectFromC<'a> {
        pub fn from_context(context: &'a ContextFromC, offset: i32) -> Self {
            let mut so: *mut c_void = std::ptr::null_mut();
            let ptrptr: *mut *mut c_void = &mut so;

            // the following line is valid compiling code also the arguments are exchanged and this
            // lets to a crash after checking res and triggering a panic:
            // let res = unsafe { create_subobject(context.inner, ptrptr, offset) };

            let res = unsafe { create_subobject(context.0.as_ptr(), offset, ptrptr) };
            if res == -1 {
                panic!("Offset={} to big must be 26 or less", offset);
            }
            // we know we can cast the void pointer:
            let inner_ptr = (unsafe { *ptrptr }) as *const c_char;
            SubObjectFromC {
                inner: inner_ptr,
                _covariant: PhantomData,
            }
        }
    }
    impl<'a> std::fmt::Display for SubObjectFromC<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let cstr = unsafe { CStr::from_ptr(self.inner) };
            write!(f, "{}", cstr.to_str().expect("Invalid UTF8"))
        }
    }
}

fn main() {
    println!("API Create, transform and Query functions\n-------");
    let context = ffi::ContextFromC::new();
    let output = if context.is_lower() {
        "lowercase"
    } else {
        "uppercase"
    };
    println!("context is {}", output);

    // some innocent operations
    let mut context = context;
    context.to_lower();
    context.to_upper();
    context.to_upper();

    // let's see what happens:
    let context = context;
    let output = if context.is_lower() {
        "lowercase"
    } else if context.is_upper() {
        "uppercase"
    } else {
        "OUT-OF-SPEC"
    };
    println!("context is {}", output);
    // context is out of spec...
    // the implementation decided the caller has to
    // ensure to_lower is only called if is_upper==1
    // sometimes you'll find these kind of invariants in
    // external APIs
    //
    // Feel free to extend the RUST Binding to never be
    // out of spec.

    // ---

    // Next we want to use subobjects:
    println!("\nSubobject\n---------");
    let mut context = ffi::ContextFromC::new();

    let so = SubObjectFromC::from_context(&context, 16);
    println!("so: {}", so);

    // uncomment to check lifetime
    //drop(context);
    //println!("so: {}", so);

    context.to_lower();
    // uncomment to see cannot borrow as mutable:
    //println!("so: {}", so);

    // Maybe we want the subobject to be a view that stays arround
    // even when the transformers `to_upper` or `to_lower` are called
    // in that case we need to re-implement with interior mutablity in
    // mind, see `std::cell::RefCell` for example.

    // ------

    // and now we change the callback and trigger it
    println!("\nCallback\n--------");
    let context = ffi::ContextFromC::new();
    unsafe { ffi::set_callback(handle_callback) };
    let _new_so = SubObjectFromC::from_context(&context, 20);
}

extern "C" fn handle_callback(subobject: *const std::ffi::c_char) {
    // transform to Rust String:
    let cstr = unsafe { std::ffi::CStr::from_ptr(subobject) };
    // cstr does not implement display, we convert to Rust String
    let rstr = match cstr.to_str() {
        Ok(utf8) => utf8,
        Err(err) => panic!("We have no valid UTF8: {}", err),
    };

    println!("Rust Callback for '{}' invoked!", rstr);

    // rstr has the same lifetime as *const c_char, in some cases a copy is needed
    let _owned_str = rstr.to_owned();
}
