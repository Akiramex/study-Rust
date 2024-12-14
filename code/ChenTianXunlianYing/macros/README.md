## Macro 宏编程

在 Rust 中宏分为两大类：声明式宏( declarative macros ) macro_rules! 和三种过程宏( procedural macros ):

+ #[derive]，在之前多次见到的派生宏，可以为目标结构体或枚举派生指定的代码，例如 Debug 特征

+ 类属性宏(Attribute-like macro)，用于为目标添加自定义的属性

+ 类函数宏(Function-like macro)，看上去就像是函数调用

## 声明宏

~~~ Rust
// my_vec! = my_vec! {1,2,3}
macro_rules! my_vec{
    () => { Vec::new() };
    ($elem:expr; $n:expr) => { std::vec::from_elem(elem, n)};
    ($($x:expr),+ $(,)?) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// ? operator
#[macro_export]
macro_rules! my_try {
    ($expr: expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.into())
        }
    };
}
~~~