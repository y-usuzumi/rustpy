mod cpython;
mod pypy;

pub enum EPyImpl {
    CPython,
    PyPy
}

pub trait PyImpl {
    fn execute(&self, code: &String) -> i8;
}

impl PyImpl for EPyImpl {
    fn execute(&self, code: &String) -> i8 {
        match *self {
            EPyImpl::CPython => cpython::execute(code),
            EPyImpl::PyPy => pypy::execute(code)
        }
    }
}
