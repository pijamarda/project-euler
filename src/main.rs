/*
    Project Euler Problems:
    https://projecteuler.net
*/

fn main() {
    println!("Project Euler");
    /*
    problem1();   
    problem2(4000000);
    */
    //       600851475143
    //       124000000   3m 3sec con terminal
    //       124000000   3sec sin terminal
    //       40000000   54secs con terminal
    //       1240000000 25sec sin terminal
    //       6008514751 25sec sin terminal
    problem3(600851475143);
}

fn problem1() {    
    let mut sum = 0;
    for i in 1..1000 {
        if (modulo(i,3) == 0) || (modulo(i,5) == 0) {
            sum += i;
        }        
    }
    println!("{}",sum);    
}

fn problem2(max: i32) {
    let mut n1 = 1;
    let mut n2 = 2;
    let mut suma = 0;
    let mut even_sum = 2;    
    while suma < max {
        suma = n1 + n2;
        n1 = n2;
        n2 = suma;            
        if (modulo(suma,2) == 0) && (suma < max) {
            print!("{} ", suma);
            //print!("{} ",even_sum);
            even_sum += suma;
        }

    }
    println!("");
    println!("{}", even_sum);
}

/*
    The prime factors of 13195 are 5, 7, 13 and 29.
    What is the largest prime factor of the number 600.851.475.143 ?    
*/
fn problem3(number_original: u64) {    
    let mut largest: u64 = 0;   
    let mut i:u64 = 2; 
    let mut number = number_original;
    while i <= number {
        //println!("Trying: {}",i);
        if number % i == 0 {
            if is_prime_u64(i) {
                print!("{} ",i);
                largest = i;
                number = number / i;
            }
        }
        i += 1;
    }    
    println!("The largest prime factor of {} is: {}", number_original, largest);
}

// Calculates modulo
fn modulo(x: i32, y: i32) -> i32 {
    return ((x % y) + y) % y;
}

// Calculates modulo
fn modulo_u64(x: u64, y: u64) -> u64 {
    return ((x % y) + y) % y;
}

// Used to help on problem 2
fn fibonacci(max: i32) {
    let mut n1 = 1;
    let mut n2 = 2;
    let mut suma = 0;    
    print!("{} ", n1);
    print!("{} ", n2);    
    while suma < max {        
        suma = n1 + n2;
        n1 = n2;
        n2 = suma;
        if suma < max {
            print!("{} ", suma);
        }
    }
    println!(""); 
}

fn is_prime(number: i32) -> bool {
    let mut prime: bool = true;
    for i in 2..(number - 1) {
        if modulo(number,i) == 0 {
            prime = false;
        }
    }
    return prime;
}

fn is_prime_u64(number: u64) -> bool {
    let mut prime: bool = true;
    for i in (2..number/2) {
        //print!("{} ", i);
        if number % i == 0 {
            prime = false;
            break;
        }
    }
    return prime;
}
