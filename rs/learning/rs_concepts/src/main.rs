fn main() {

    fibonacci(&50);

} 

fn fibonacci(n_fibo: &i32) {
    let mut n1: i64 = 0;
    let mut n2: i64 = 1;
    let mut fibo: i64 = 0;

    let mut counter: i32 = 0;  
     
    while counter < n_fibo {
      
        println!("current fibo {cfib}: {fib}", cfib=counter, fib=fibo);
          
        fibo = n1 + n2;
        
        n1 = n2;
        n2 = fibo;
        
        counter += 1;
     
    }  
    
}

