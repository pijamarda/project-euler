/*
    Project Euler Problems:
    https://projecteuler.net
*/

fn main() {
    println!("Project Euler");
<<<<<<< HEAD
    problem3(600851475143);
=======
>>>>>>> parent of c54b8c3... added comments on the introduction of the problems
    /*
    problem1();   
    problem2(4000000);
    problem3(600851475143);
    problem4(56);
    */
    problem5();
}

#[allow(dead_code)]
fn problem1() {    
    let mut sum = 0;
    for i in 1..1000 {
        if (modulo(i,3) == 0) || (modulo(i,5) == 0) {
            sum += i;
        }        
    }
    println!("{}",sum);    
}

#[allow(dead_code)]
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
#[allow(dead_code)]
fn problem3(number_original: u64) {    
    let mut largest: u64 = 0;   
    let mut i:u64 = 2; 
    let mut number = number_original;
<<<<<<< HEAD
    while i <= number {        
        if number % i == 0 {            
            largest = i;
            number = number / i;            
=======
    while i <= number {
        //println!("Trying: {}",i);
        if number % i == 0 {
            if is_prime_u64(i) {
                print!("{} ",i);
                largest = i;
                number = number / i;
            }
>>>>>>> parent of c54b8c3... added comments on the introduction of the problems
        }
        i += 1;
    }    
    println!("The largest prime factor of {} is: {}", number_original, largest);
}

/*
    A palindromic number reads the same both ways. 
    The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
    Find the largest palindrome made from the product of two 3-digit numbers.
*/
#[allow(dead_code)]
fn problem4(num: i32) {
    check_palindrome(num);
}

/*
    Smallest multiple
    Problem 5
    2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
    What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/
fn problem5() {
    let mut found = false;
    let mut number = 1;
    let mut i = 1;
    while !found {        
        while i <= 20 {
            if number % i == 0 {                
                i += 1;                
            }
            else {
                i = 1;
                number += 1;                
                continue;
            }
        }        
        found = true;
        println!("The number is: {}", number);        
    }
}

fn check_palindrome(num: i32) -> bool {
    let mut is_palindrome = false;
    let num_text = num.to_string();
    println!("{}", num_text);
    if is_palindrome {
        is_palindrome=true;
    }
    return is_palindrome;
}

// Calculates modulo
fn modulo(x: i32, y: i32) -> i32 {
    return ((x % y) + y) % y;
}

// Used to help on problem 2
#[allow(dead_code)]
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

#[allow(dead_code)]
fn is_prime(number: i32) -> bool {
    let mut prime: bool = true;
    for i in 2..(number - 1) {
        if modulo(number,i) == 0 {
            prime = false;
        }
    }
    return prime;
}

#[allow(dead_code)]
fn is_prime_u64(number: u64) -> bool {
    let mut prime: bool = true;
    for i in 2..number/2 {
        //print!("{} ", i);
        if number % i == 0 {
            prime = false;
            break;
        }
    }
    return prime;
}
