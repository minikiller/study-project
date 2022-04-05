<link rel="stylesheet" type="text/css" href="auto.css" />

# rust 重要概念

## 所有权

## struct

* 一旦struct的实例是可变的，那么它所有的字段都是可变的

## 字符串

* String是一个可变的、堆上分配的UTF-8的字节缓冲区。可以通过push_str附加字符串。
* 而str是一个不可变的固定长度的字符串，如果是从String解引用而来的，则指向堆上，如果是字面值，则指向静态内存。
* &String 是String的borrowed类型，这只不过是一个指针类型，可以传递而不放弃ownership。事实上，一个&String可以当做是&str。
```rust
fn main() {
    let s = String::from("Hello, Rust!");
    foo(&s);
}
fn foo(s: &str) {
    println!("{}", s);
}
```

### 关于&String
&String 是String的borrowed类型，这只不过是一个指针类型，可以传递而不放弃ownership。事实上，一个&String可以当做是&str。

```rust
 fn main() {
    let s = String::from("Hello, Rust!");
    foo(&s);
}
fn foo(s: &str) {
    println!("{}", s);
}
```

如果我们想修改字符串的内容，只需要传递一个可变引用就行了。

```rust
fn main() {
    let mut s = String::from("Hello, Rust!");
    foo(&mut s);
}
fn foo(s: &mut String) {
    s.push_str("appending foo..");
    println!("{}", s);
}
```

### &str => String

```rust
let a = "hello";
let c = a.to_string();
let d = String::from(a);
let d = a.to_owned();
```

### String => &str

```rust
let e = &String::from("Hello Rust");
// 或使用as_str()
let e_tmp = String::from("Hello Rust");
let e = e_tmp.as_str();
// 不能直接这样使用 
// let e = String::from("Hello Rust").as_str();
```
###  String + &str => String

```rust
let mut strs = "Hello".to_string();
// let mut strs = String::from("Hello");
strs.push_str(" Rust");
println!("{}", strs);
```
> 总结
如果只想要一个字符串的只读视图，或者&str作为一个函数的参数，那就首选&str。如果想拥有所有权，想修改字符串那就用String吧。
