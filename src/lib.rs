//! Helper functions for retrieving stdin, stdout, stderr to work with `libc`.
#![warn(missing_docs)]
#![no_std]

extern crate libc;

use libc::FILE;

extern "C" {
    /// expression of type FILE* associated with the input stream
    #[link_name = "libc_stdhandle_rs_stdin"]
    pub fn stdin() -> * mut FILE;

    /// expression of type FILE* associated with the output stream
    #[link_name = "libc_stdhandle_rs_stdout"]
    pub fn stdout() -> * mut FILE;

    /// expression of type FILE* associated with the error output stream 
    #[link_name = "libc_stdhandle_rs_stderr"]
    pub fn stderr() -> * mut FILE;
}