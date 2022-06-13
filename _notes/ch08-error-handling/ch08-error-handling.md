## 8-2 不可恢复的错误 (11:18)

异常, 程序无法继续执行

```
panic!("error");

assert!(1==2);
assert_eq!(1,2);

unimplemented!();

unreachable!();
```

## 8-3 可恢复的错误 (06:47)

Result<T, E>

```
let r = read();
match r {
  Ok(data) => {

  },
  Err(err) => {

  }
}

```

粗暴处理 result

```
let r = read().unwrap();
```

## 8-4 自定义错误与问号表达式 (13:38)

? 将 Result 中的正常值取出，将错误传递到函数外，并终止函数执行

```
  fn foo() -> Result<T, E> {
    let x = bar()?;
  }
```

把 library 中的 error 包装成自己的 error

```

```
