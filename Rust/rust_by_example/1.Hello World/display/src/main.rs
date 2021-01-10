fn main() {
    println!("Hello, world!");
    println!("---------------------------------");

    use std::fmt;

    // 自定义 Debug 打印格式
    struct Structure(i32);
    impl fmt::Display for Structure {
        fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
            write!(format, "{}", self.0)
        }
    }

    // 实例
    #[derive(Debug)]
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
            write!(format, "({}, {})", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }
    impl fmt::Display for Point2D {
        fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
            write!(format, "x: {}, y: {}", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!(
        "The big_range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );
    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare pointes:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // List
    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;

            write!(format, "[")?;

            for (index, value) in vec.iter().enumerate() {
                if index != 0 {
                    write!(format, ",",)?;
                }
                write!(format, "{}", value)?
            }
            write!(format, "]")
        }
    }

    // Activity
    struct ListActivity(Vec<i32>);
    impl fmt::Display for ListActivity {
        fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(format, "[")?;

            for (index, value) in vec.iter().enumerate() {
                if index != 0 {
                    write!(format, ", ")?;
                }
                write!(format, "{}: {}", index, value)?;
            }
            write!(format, "]")
        }
    }
    let v_activity = ListActivity(vec![1, 2, 3]);
    println!("{}", v_activity);
}
