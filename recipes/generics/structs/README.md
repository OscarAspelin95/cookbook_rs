# Problem
You want to create a struct that holds a generic type, enabling e.g., sorting a `Vec<MyStruct<T>>`.

# Solution
Create a struct that is general over a type `T` that implements e.g., `PartialOrd`.
