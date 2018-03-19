const A_CONST: isize = 127_2;
const B_CONST: isize = 0_3;

#[derive(Debug)]
struct Rectangle {
    length: f32,
    width: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn is_bigger(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }

    fn diagonal(rect: &Rectangle) -> f32 {
        return f32::sqrt(f32::powi(rect.length, 2) + f32::powi(rect.width, 2));
    }
}

enum Shape {
    Square(u64),
    Rectangle {length: u32, width: u32},
    Circle(u32),
}

fn main() {
    println!("Hello, world! {}", A_CONST+B_CONST);
    let rect = Rectangle {length:50.0, width:20.0};
    //println!("{:?}", rect);
    //println!("{:#?}", rect);
    //println!("{}", rect.area());
    //println!("{}", rect.is_bigger(&Rectangle{length:20.0, width: 30.0}));
    //println!("{}", Rectangle::diagonal(&rect));
    //println!("Enter the number :");
    //let mut x = String::new();
    //std::io::stdin().read_line(&mut x)
    //                .expect("read line failed");
    //let x: u64 = match x.trim().parse() {
    //    Ok(num) => num,
    //    Err(_) => 2,
    //};
    //println!("fibonacci output : {}", fib(x));
    //let s = String::from("hello");
    //let s2 = String::from("jello world");
    //let rs = first_word(&s);
    //let rs2 = first_word(&s2);
    //let rc = first_word("nello world");
    //println!("{}", rs);
    //println!("{}", rs2);
    //println!("{}", rc);
    println!("{}", karatsuba(2, 4));
    println!("{}", karatsuba(23, 34));
    println!("{}", karatsuba(1234, 5678));
    println!("{}, {}", num_digits(123), num_digits(5678));
    println!("{}", karatsuba(123, 5678));
}

fn num_digits(i: u64) -> u32 {
    let mut max:u32 = 19;
    while max >= 1 {
        if i > u64::pow(10, max - 1) && i < (u64::pow(10, max) - 1) {
            return max;
        }
        max -= 1;
    }
    1
}

fn karatsuba(i: u64, j:u64) -> u64 {
    let (n, nj) = (num_digits(i), num_digits(j));
    match n {
        1 => i*j,
        nj => {
            let (a,b,c,d) = (i / u64::pow(10, (n/2)),
                            i % u64::pow(10, (n/2)),
                            j / u64::pow(10, (n/2)),
                            j % u64::pow(10, (n/2)));
            let ac = karatsuba(a, c);
            let bd = karatsuba(b, d);
            let prod = karatsuba(a+b, c+d) - ac - bd;
            return u64::pow(10, n)*ac + u64::pow(10, n/2)*prod + bd;
        }
    }
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item==b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn fib(x: u64) -> u64 {
    println!("recursion is bollocks in this case. Trust me!!");
    let mut a: u64 = 1;
    let mut b: u64 = 1;
    for _ in 2..x {
        let c: u64 = b;
        b = a + b;
        a = c;
    }
    b
}
