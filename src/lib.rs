extern crate libc;

mod py_impl;

#[cfg(test)]
mod tests {
    use py_impl;
    use py_impl::PyImpl;

    const TEST_CODE: &str = "print 'Hello world'";

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn unsafe_code() {
        let x: i32 = 5;
        let p: *const i32 = &x;
        unsafe {
            assert_eq!(*p, x);
        }
        assert_eq!(unsafe { *p }, x);
    }

    #[test]
    fn py_impl_cpython() {
        let ret = py_impl::EPyImpl::CPython.execute(&TEST_CODE.to_owned());
        assert_eq!(0, ret);
        assert_eq!(0, ret);
    }

    #[test]
    fn py_impl_pypy() {
        let ret = py_impl::EPyImpl::PyPy.execute(&TEST_CODE.to_owned());
        assert_eq!(0, ret);
        assert_eq!(0, ret);
    }
}
