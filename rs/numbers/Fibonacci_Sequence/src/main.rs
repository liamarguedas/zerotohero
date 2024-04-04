fn main() {
    generateFibo(50);
}

fn generateFibo(n_fibos: i32) {

    let mut n1: i128 = 0;
    let mut n2: i128 = 1;
    let mut fibo: i128 = 0;
    
    let mut counter: i32 = 0;
    
    while counter <= n_fibos {

        println!("{}", fibo);

        n1 = n2;
        n2 = fibo;

        fibo = n1 + n2;
    }
} 
