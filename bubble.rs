fn main() {
    let a = 2; 
    let b = 2; 

    let c = a + b; 

    println!("{}",c);

    let d; 
    d = a; 

    println!("{}", d == a);

    let e = &d as *const i32;
    let f = &a as *const i32; 

    println!("{:?}", e);
    println!("{:?}", f);

    for _ in 0..10 {
        println!("{}", "glizzy");
    }


    for x in 0..11 {
        for y in x..11 {
            println!("{:?}", (x,y))
        } 
    }
    

    for x in 0..15 {
        for _ in 0..x {
            print!("*");
        }
        print!("\n");
    }
    
    // ariana grande 
    let j = ['a', 'r', 'i', 'a', 'n', 'a', 'g', 'r', 'a', 'n', 'd', 'e'];
    for x in 0..j.len()+1 {
        for y in 0..x {
            print!("{} ", j[y]);
        }
        print!("\n");
    }


}