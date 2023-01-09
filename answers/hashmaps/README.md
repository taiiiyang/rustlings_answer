# Hashmaps
A *hash map* allows you to associate a value with a particular key.
You may also know this by the names [*unordered map* in C++](https://en.cppreference.com/w/cpp/container/unordered_map), 
[*dictionary* in Python](https://docs.python.org/3/tutorial/datastructures.html#dictionaries) or an *associative array* in other languages.

This is the other data structure that we've been talking about before, when
talking about Vecs.

## Further information

- [Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

### HashMap
and_modify 传递一个闭包，如果查找成功返回 value 的可变引用，不成功调用 ``or_insert`` 方法进行插入
``remove``方法移除一项
if two keys are equal, their hashes must be equal.


