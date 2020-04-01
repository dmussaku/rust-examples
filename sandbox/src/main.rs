#![allow(dead_code)]

use std::collections::HashMap;

fn match_examples(){
    let country_code = 999;

    let country = match country_code {
        44 => "UK",
        49 => "DE",
        7 =>  "KZ",
        1..=999 => "unknown",
        _ => "invalid"
    };

    println!("{}", country);
}

struct Point{
    x: f32,
    y: f32
}
fn struct_example(){
    let p = Point{x: 3.00, y: 5.00};
    println!("Point at ({}, {})", p.x, p.y);
}

enum Color{
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    Cmyk{cyan:u8, magenta:u8, yellow:u8, black:u8}, // struct
}
fn enums_example(){
    // let c:Color = Color::RgbColor(0, 0, 0);
    let c:Color = Color::Cmyk{cyan: 0, magenta: 0, yellow: 0, black:255};
    match c{
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0)
        | Color::Cmyk{cyan: _, magenta: _, yellow: _, black: 255}=> println!("black"),
        Color::RgbColor(r, g, b) => println!("RgbColor: ({}, {}, {})", r, g, b),
        _ => println!("not r")
    }
}

fn vector_example(){
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);

    let i:usize = 0;  // has to be of type unsigned size
    println!("a[0] = {}", a[i]);

    let j:usize = 2;

    match a.get(j){
        Some(_x) => println!("a[{}] = {}", j, a[j]),
        _ => println!("No such element")
    }
}

fn use_slice(slice: &mut[i32]){
    println!("First elem {}, len = {}", slice[0], slice.len());
}
fn slices_example(){
    let mut data = [1,2,3,4,5];

    use_slice(&mut data[1..4]);
}


fn string_example(){
    // utf-8
    // let s:&'static str = "Hello World"; // &str = string slice
    // for ch in s.chars(){
    //     print!("{}", ch);
    //     if let Some(last_char) = s.chars().last(){
    //         println!();
    //     }
    // }

    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8){
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);
}


fn _sum_and_product(x:i32, y:i32) -> (i32, i32){
    (x+y, x*y)
}


fn tuples_example(){
    let x = 3;
    let y = 4;

    let sp = _sum_and_product(x, y);
    let combine = (sp, sp);
    println!("{:?}", (combine.1).1);
}

fn hash_maps_example(){
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);
    shapes.insert("square".into(), 5);
    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes
            .entry("circle".into())
            .or_insert(1);
        *actual = 0;
    }

    for (key, value) in &shapes{
        println!("{:?}: {}", key, value);
    }
}


// fn how_many(x:i32) -> &'static str{
//     match x{
//         0 => "no",
//         1 | 2 => "one or two",
//         z@ 9...11 => "a lot of",
//         12 => "a dozen",
//         _ if (x % 2 == 0) => "some",
//         _ => "a few"
//     }
// }
//
// fn pattern_matching(){
//     for x in 0..13{
//         println!("{}: I have {} oranges", x, how_many(x));
//     }
//
//     let point = (3,4);
//     match (point) {
//         (0,0) => println!("origin"),
//         (0, y) => println!("x axis, y = {}", y),
//         (x, 0) => println!("x = {}, y axis", x),
//         (x, y) => println!("x = {}, y = {}",x,y)
//     }
// }



struct PointT<T>{
    x: T,
    y: T
}
fn generics_example(){
    let _a:PointT<i32> = PointT{x: 0, y: 4};
    let _b:PointT<f32> = PointT{x: 1.2, y: 1.2};
}


fn mutable_increase(x: &mut i32){
    *x += 1;
}


fn functions_example(){
    let mut x = 1;
    mutable_increase(&mut x);
    println!("x = {}", x);
}


struct Line{
    start: Point,
    end: Point
}

impl Line{
    fn length(&self) -> f32{
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;

        (dx*dx + dy*dy).sqrt()
    }
}


fn methods_example(){
    let start = Point{x: 1.0, y: 1.0};
    let end = Point{x: 5.0, y:5.0};

    let line = Line{start, end};
    println!("line length = {}", line.length());
}


fn main() {
    // match_examples();
    // struct_example();
    // enums_example();
    // process_union_value(IntOrFloat{i: 300})
    // option_example();
    // vector_example();
//     slices_example();
//     string_example();
//     tuples_example();
//     hash_maps_example();
//     pattern_matchi ng();
//     generics_example();
//     functions_example();
//     methods_example();
}
