fn main() {
    let sum = 5 + 10;
    let difference = 10 - 2;
    let a = [1,2,3,4,5];
    let b: [i32; 5] = [1,2,3,4,5];
    let c = [3; 5]; // -> equal to let c = [3,3,3,3,3];
    println!("{sum} {difference} {:?} {:?} {:?}", a, b, c);
}
