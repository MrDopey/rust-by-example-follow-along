use std::fmt::{self, Display};

pub fn chapter_one() {
  // hello_word();
  // positional_args();
  // playing_with_structs();
  // params_and_display();
  custom_display_function();
} 

fn hello_word() {
    // Statements here are executed when the compiled binary is called.
  
    // Print text to the console.
    println!("Hello World!");
    println!("I'm a Rustacean!");
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