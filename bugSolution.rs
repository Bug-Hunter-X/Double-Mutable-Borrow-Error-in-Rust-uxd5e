fn main() {
    let mut x = 5;
    { // Creating a block to limit the scope of y
        let y = &mut x;
        *y += 1; 
    }
    { // Creating a block to limit the scope of z
        let z = &mut x;
        *z +=1;
    }
    println!("x = {}", x);
}