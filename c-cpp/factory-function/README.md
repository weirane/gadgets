# factory-function
This is an example of factory function in C++. After using Rust for some months, I find this kind of design pattern very satisfying, since the constructor overload in C++ is so confusing. For example, if I ask you what is `vec` when it is defined like this:
```cpp
std::vector<int> vec(92, 2);
```
you have to check the documentation to find out what `vec` is.

Having clearer "constructors" is just one of the advantages of using factory functions. It can also solve the problems when you cannot overload two constructors. For example, the arguments of the constructors are of the same type. See `point.cpp` for an example. Moreover, with factory functions, you can define an interface for creating an object, but let subclasses decide which class to instantiate, example in `animal.cpp`.

Ref:
- [Perils of Constructors](https://matklad.github.io/2019/07/16/perils-of-constructors.html)
- [Factory Method Design Pattern](https://sourcemaking.com/design_patterns/factory_method)
