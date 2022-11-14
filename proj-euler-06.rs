//  PROBLEMA 6
//The sum of the squares of the first ten natural numbers is,
//  1² + 2² + ... + 10² = 385
//
//The square of the sum of the first ten natural numbers is,
//  (1 + 2 + ... + 10)² = 55² = 3025
//
//Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025-385=2640.
//Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn main() {
    let n = 100;

    let s1 = suma_euler(n);
    let s1_sq = s1 * s1;
    // println!("Cuadrado de la suma ({}) = {}", n, s1_sq);

    let s2 = suma_sq(n);
    // println!("Suma de cuadrados ({}) = {}", n, s2);
    
    println!("Solución: {}", s1_sq - s2);
}

fn suma_euler(num: i32) -> i32 {
    // Suma euleriana
    num * (num + 1) / 2
}

fn suma_sq(num: i32) -> i32 {
    let mut suma = 0;

    for i in 1..num+1 {
        suma += i * i;
    }
    suma
}
