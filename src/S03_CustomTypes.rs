
pub fn chapter_three() {
  structures();
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
