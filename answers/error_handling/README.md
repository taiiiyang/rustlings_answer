# Error handling
Most errors aren’t serious enough to require the program to stop entirely. 
Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to.
For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

## Further information

- [Error Handling](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [Boxing errors](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)

## errors1
使用 ``Result<String, &'static str>`` 来优化错误时的处理，错误字符串标注全局声明周期