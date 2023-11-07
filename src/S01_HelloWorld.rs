use std::{fmt::{self, Display, Formatter}};

pub fn chapter_one() {
  // hello_word();
  // positional_args();
  // playing_with_structs();
  // params_and_display();
  // custom_display_function();
  // testcase_list();
  formatting();
}

fn positional_args() {
  
  let x = 5 + 5;
  // println!("Is 'x' 10 or 100? x = {} {}", x)

  // println!("{0}, this is {} {} {}. {1}, this is {0}", "Alice", "Bob");

  // As can named arguments.
  println!("{subject} {verb} {object} || {2}",
           object="the lazy dog",
           subject="the quick brown fox",
           verb="jumps over");

}

fn playing_with_structs() {
  

  #[allow(dead_code)] // disable `dead_code` which warn against unused module
  #[derive(fmt::Debug)]
  struct Structure(i32);

  // struct DebugPrintable(i32);

  #[derive(fmt::Debug)]
  struct Deep(Structure);

  let x = Structure(3);
  let y = Deep(x);
  println!("Hello {:#?}", y);

  println!("{:?} months in a year.", 12);

  println!("{1:?} {0:?} is the {actor:?} name.",
  "Slater",
  "Christian",
  actor="actor's");
  // Display(Structure(3));

  impl Display for Structure {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(f, "{:?}", self.0)
      }
  }

  // This will not compile because `Structure` does not implement
  // fmt::Display.
  println!("This struct `{}` won't print...", Structure(3));
}

fn params_and_display() {
  
  let axle: f64 = 1.0;
  let width: usize = 5;
  println!("{axle} is here and {width} is there");

  let pi: f64 = 3.141592;

  println!("pi is {pi:.2}");
}

fn custom_display_function() {
  // A structure holding two numbers. `Debug` will be derived so the results can
  // be contrasted with `Display`.
  #[derive(Debug)]
  struct MinMax(i64, i64);

  // Implement `Display` for `MinMax`.
  impl fmt::Display for MinMax {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          // Use `self.number` to refer to each positional data point.
          write!(f, "({:.3}, {})", self.0, self.1)
      }
  }

  println!("{}", MinMax(10, 23));

  // Define a structure where the fields are nameable for comparison.
  #[derive(Debug)]
  struct Point2D {
      x: f64,
      y: f64,
  }

  // Similarly, implement `Display` for `Point2D`.
  impl fmt::Display for Point2D {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          // Customize so only `x` and `y` are denoted.
          write!(f, "x: {}, y: {}", self.x, self.y)
      }
  }

  let point = Point2D { x: 3.3, y: 7.2 };

  println!("Compare points:");
  println!("Display: {}", point);
  println!("Debug: {:?}", point);

  // Error. Both `Debug` and `Display` were implemented, but `{:b}`
  // requires `fmt::Binary` to be implemented. This will not work.
  // println!("What does Point2D look like in binary: {:b}?", point);

  #[derive(Debug)]
  struct ComplexStruct {
      real: f64,
      imag: f64,
  }

  impl Display for ComplexStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{:.1} + {:.1}i", self.real, self.imag)
    }
}

  let number = ComplexStruct { real: 3.3, imag: 7.2 };

  println!("Compare points:");
  println!("Display: {}", number);
  println!("Debug: {:?}", number);


}
fn testcase_list() {
    // Define a structure named `List` containing a `Vec`.
  struct List(Vec<i32>);

  impl fmt::Display for List {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          // Extract the value using tuple indexing,
          // and create a reference to `vec`.
          let vec = &self.0;

          write!(f, "[")?;

          // Iterate over `v` in `vec` while enumerating the iteration
          // count in `count`.
          for (count, v) in vec.iter().enumerate() {
              // For every element except the first, add a comma.
              // Use the ? operator to return on errors.
              if count != 0 { write!(f, ", ")?;}
              write!(f, "{}: {}", count, v)?;
          }

          // Close the opened bracket and return a fmt::Result value.
          write!(f, "]")
      }
  }

  let v = List(vec![1, 2, 3]);
  println!("{}", v)
} 
fn formatting() -> () {
  struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
  }


  impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            // `write!` is like `format!`, but it will write the formatted string
            // into a buffer (the first argument).
            write!(f, "{}: {:.3}°{} {:.3}°{}",
                  self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }


    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl Display for Color {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
           write!(f, "RGB ({0}, {1}, {2}) 0x{0:0<2X}{1:0<2X}{2:0<2X}", self.red, self.green, self.blue)
        }
    }


    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", color);
    }


}
