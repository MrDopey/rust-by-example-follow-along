use std::fmt;
use std::convert::{From, Into};
use std::convert::TryFrom;
use std::convert::TryInto;

use crate::S06_Conversion;

pub fn chapter_six() {
  // from_and_to();
  // tryfrom_and_tryto();
  to_and_from_string();
}

fn to_and_from_string() {
  
struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}
  let circle = Circle { radius: 6 };
  println!("{}", circle.to_string());



    let parsed: f32 = "5".parse().unwrap();
    let turbo_parsed = "abc".parse::<f32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

fn tryfrom_and_tryto() {
  
#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}


    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

      #[derive(Debug)]
      struct Number {
          value: i32,
      }
fn from_and_to() {

  // {

  //     impl From<i32> for Number {
  //         fn from(item: i32) -> Self {
  //             Number { value: item }
  //         }
  //     }
  //   let num = Number::from(30);
  //   println!("My number is {:?}", num);
  // }
{

  
impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}


    let int = 5;
    let num: Number= int.into();
    println!("My number is {:?}", num);

}
}