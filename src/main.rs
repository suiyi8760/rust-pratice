fn apply(v: i32, f: fn(i32) -> i32) -> i32 {
    f(v)
}

fn square(v: i32) -> i32 {
    v * v
}

fn double(v: i32) -> i32 {
    v+v
}

fn main() {
    
    let sum = {
        apply(10,square) + apply(20,double)
    };

    println!("result: {}",sum);
}
