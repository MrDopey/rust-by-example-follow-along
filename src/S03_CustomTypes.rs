// Globals are declared outside all other scopes.
static mut LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

pub fn chapter_three() {
  // structures();
  // enums();
  // enum_use();
  // enum_clike();
  // enum_linkedList();
  constants();
}

fn constants() {

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}
    let n = 16;
    unsafe { LANGUAGE = "box"; }

    // Access constant in the main thread
    unsafe { println!("This is {}", LANGUAGE); }
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
}

fn enum_linkedList() {

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

  use List::*;

// Methods can be attached to an enum
  impl List {
      // Create an empty list
      fn new() -> List {
          // `Nil` has type `List`
          Nil
      }

      // Consume a list, and return the same list with a new element at its front
      fn prepend(self, elem: u32) -> List {
          // `Cons` also has type List
          Cons(elem, Box::new(self))
      }

      // Return the length of the list
      fn len(&self) -> u32 {
          // `self` has to be matched, because the behavior of this method
          // depends on the variant of `self`
          // `self` has type `&List`, and `*self` has type `List`, matching on a
          // concrete type `T` is preferred over a match on a reference `&T`
          // after Rust 2018 you can use self here and tail (with no ref) below as well,
          // rust will infer &s and ref tail. 
          // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
          match *self {
              // Can't take ownership of the tail, because `self` is borrowed;
              // instead take a reference to the tail
              Cons(_, ref tail) => 1 + tail.len(),
              // Base Case: An empty list has zero length
              Nil => 0
          }
      }

      // Return representation of the list as a (heap allocated) string
      fn stringify(&self) -> String {
          match *self {
              Cons(head, ref tail) => {
                  // `format!` is similar to `print!`, but returns a heap
                  // allocated string instead of printing to the console
                  format!("{}, {}", head, tail.stringify())
              },
              Nil => {
                  format!("Nil")
              },
          }
      }
  }


    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

fn enum_clike() {
    enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}


    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}


enum Work {
    Civilian,
    Soldier,
}
fn enum_use() {
    // An attribute to hide warnings for unused code.

enum Status {
    Rich,
    Poor,
}
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    use crate::S03_CustomTypes::Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }

}

fn enums() {
    // Create an `enum` to classify a web event. Note how both
  // names and type information together specify the variant:
  // `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
  // Each is different and independent.
  enum WebEvent {
      // An `enum` variant may either be `unit-like`,
      PageLoad,
      PageUnload,
      // like tuple structs,
      KeyPress(char),
      Paste(String),
      // or c-like structures.
      Click { x: i64, y: i64 },
  }

  // A function which takes a `WebEvent` enum as an argument and
  // returns nothing.
  fn inspect(event: WebEvent) {
      match event {
          WebEvent::PageLoad => println!("page loaded"),
          WebEvent::PageUnload => println!("page unloaded"),
          // Destructure `c` from inside the `enum` variant.
          WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
          WebEvent::Paste(s) => println!("pasted \"{}\".", s),
          // Destructure `Click` into `x` and `y`.
          WebEvent::Click { x, y } => {
              println!("clicked at x={}, y={}.", x, y);
          },
      }
  }

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    // Creates a type alias
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Subtract;
     
     
    impl Operations {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }

    let a = x.run(2, 3);
    println!("{}", a);
}

fn structures() {
  #[derive(Debug)]
  struct Person {
      name: String,
      age: u8,
  }

  // A unit struct
  struct Unit;

  // A tuple struct
  struct Pair(i32, f32);

  // A struct with two fields
  #[derive(Debug)]
  struct Point {
      x: f32,
      y: f32,
  }

  // Structs can be reused as fields of another struct
  #[derive(Debug)]
  struct Rectangle {
      // A rectangle can be specified by where the top left and bottom right
      // corners are in space.
      top_left: Point,
      bottom_right: Point,
  }


    // Create struct with field init shorthand
    let name = String::from("Peter");
    // let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    // let Point { x: left_edge, y: top_edge } = point;
    let Point { x, y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: x, y: y },
        bottom_right: bottom_right,
    };
    println!("{:?}", _rectangle);
    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);


    let something = rect_area(Rectangle { top_left: Point { x: 4f32, y: 10f32 }, bottom_right: Point { x: 10f32, y: 2f32 } });
    println!("{:?}", something);

    fn rect_area(rect: Rectangle) -> f32 {
      let Rectangle { top_left: Point { x: tlx, y: tly }, bottom_right: Point { x: brx, y: bry } } = rect;

      let width = brx - tlx;
      let height = tly - bry;
      width * height
    }
    
    fn square(point: Point, size: f32) -> Rectangle {
      let Point { x: samuel, y } = point;
      Rectangle { top_left: point, bottom_right: Point { x: samuel + size, y: y - size } }
    }

    let otherThing = square(Point { x: 2f32, y: 3f32}, 4f32);
    println!("{:?}", otherThing);
}
