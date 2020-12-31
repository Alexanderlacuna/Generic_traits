##  Generics
let assume you have a function that you can reuse for different types of data.In a normal case you would have to create a function for each.Oops!that would cause unnnecessary repetition
rust offers generic to hanlde this

``` rust
 fn <T> loop_items(slice:Vec<T>){
     for item in slice.iter(){
        //  do something
     }
 }

```


### demystifying generics

- Developers should always try to minimize repetition Is this possible in rust?isn't rust a statically typed language  where the type must be known as compile time?With rust yes it is with generics which enabled us to pass dynamic type
for example we can pass an argument of a type that is dynamic


``` rust
fn  pass_generic_type <T> (item1:T){

}
```
in the above function we pass type T
what will happen at compile time?
at compile time the complile time the compile replaces each generic type with the concrete argument provided af fancy term called **Monomorphization** in rust

for example 

``` rust
fn pass_generic_type(item1:i32){
}

fn pass_generic_type(item1:String)
fn pass_generic_type(item1:f64)

```
#### returning generics types
``` rust
fn pass_generic_type<T>()->T {

}
```
### limiting the generic type

you can always limit the generic type with traits

for example let say you have a trait 
``` rust
trait MyTrait{}
```

you can make sure that the generics type passed has implemented the trait as shown below

we use these T:Mytrait syntax


``` rust
fn pass_generic_type<T:MyTrait>(item:T){}
```


**using the where clause **

you can also achieve the above using a where a clause 


``` rust


struct Point <T>{
    a:T,
    b:T
}

struct PointB<T,R>{
    a:T,
    b:R
}
fn pass_generic_type<T>(item:T)
where 
    T:MyTrait
{

}


```



## Traits

traits is similar to what we refer to some languages as interface.
the enable us implement common behaviour among different types
let say you have custom type of Rectangle and Circle they both share some traits for example

- find area
- find perimeter


``` rust
struct Rectangle {};
struct Circle {};

trait MyTrait {

    fn find_area(&self)->f64;
}


// you have to implement the trait for your type  the general syntax is
//impl Trait_Name for Type {}

impl MyTrait for Circle {
    find area(&self)->f64{
        pi*(self.radius**2)
    }
}

impl MyTrait for Rectangle {
    find_area(&self)->f64{
        2*(self.length*self.width)
    }
}

```




### Lifetimes

lifetime denotes the validity of the reference and  prevent dangling references (reference that point to nothing)
example of a dangling reference

``` rust

let r;

{

    let t=5; //scope of t starts here
    r=&t;

//t id dropped  here leaving a dangling reference
}

```

the above would cause compile time error the value t does not ling past the curly braces so that would leave r with a dangling reference



Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.


### Lifetime elision rules

1. each  input reference is assigned its own lifetime

``` rust

fn multiple_reference_inputs(item1:&str,item2:&str)->&str

fn multiple_reference_inputs<'a,'b>(item1:&'a str,item2:&'a str)

```


2. if there is only one reference in input the output is assigned the same lifetime 
for example

``` rust
let one_input_reference(item:&str)->str
// in this case if let lifetime of item is 'a the output will have the 
//sa,e lifetime
```
3. if there multiple reference and one of them &self or & mut self
the output reference is assigned the same  lifetime as self

for example

``` rust
  struct Item <'a>{
      x:&'a str
  }
  
  impl <'a> Item <'a>{
      fn caller(&self,item:&str)->&str
    //   in this case the output will be assigned the same lifetime as self
  }
```