use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");

    let point=Point{length:1,width:5};


    match point{
        Point{length:4,width:5}=>println!("hello"),
        Point{length,width}=>println!("hello")
    }
}


fn find_max<'a,T>
(slice:&'a [T])->&'a T
where
T: PartialOrd{
let mut max_value=&slice[0];

for val in slice.iter(){
    if *val>*max_value{
        max_value=val
    }
}

max_value

}

trait MyTrait{
    fn get_something(&self)->();

}
fn impl_something(items:&  (impl Clone+MyTrait)){

}

impl MyTrait for i32{
    fn get_something(&self){
     println!("the value is {}",self)
    }
  
}

impl <T,W>  MyTrait for Point<T,W>{
    fn get_something(&self) ->() {
        todo!()
    }

}

fn error_test()->Result<i32,Box<dyn std::error::Error>>{

    let results:i32="sdf".trim().parse()?;
    Ok(results)

}


// generic struct

struct Point <T,W>{
    length:T,
    width:W
}

impl <T,W> Point<T,W>{
    fn new (length:T,width:W)->Self{

        Point {
            length,
            width
        }

    }
}


// implement for i32

impl Point<i32,i32>{
    fn create_new (length:i32,width:i32)->Self{
        Point {
            length,
            width
        }
    }
}

#[cfg(test)]


mod tests {

    use super::find_max;
    #[test]

    fn test_numbers(){

        let nums=[5,1,2,3,7];
        assert_eq!(find_max(&nums),&7);

    }

    #[test]
    fn test_chars(){
        let mut chars:Vec<char>="abc".chars().collect();

        assert_eq!(find_max(&chars),&chars[2])
    }


}


