编译器提供`dead_code`
[*lint*](https://en.wikipedia.org/wiki/Lint_%28software%29)（无效代码检查），对未被使用的函数提出警告。使用属性`#[allow(dead_code)]`可以禁用此项检查。

{unused.play}

注意，在真实的程序中，你应当消除无效代码。在上述示例中，我们允许一部分无效代码，仅仅只是为了展示这一功能。
