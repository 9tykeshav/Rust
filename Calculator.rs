//main function
fn main() {
    //vector for storing sums to make a small cache like thing.
    let mut sum_cache: Vec<(i32, i32)> = vec![(30, 50)];
    // cache vector for substraction
    let mut minus_cache: Vec<(i32, i32)> = vec![];

    let num1 = 30; // value for 1st number

    let num2 = 20; // value for 2nd number
    let ans = sum(num1, num2, &mut sum_cache);
    println!("{:?}", ans);
}

// addition function
fn sum(numb1: i32, numb2: i32, vect: &mut Vec<(i32, i32)>) -> (i32, i32) {
    let cache_val = vect.iter().find(|(num1, _)| *num1 == numb1);
    // trying to find the value in vector
    if cache_val != None && cache_val.unwrap().0 == numb1 {
        return *cache_val.unwrap();
    //if found return it
    } else {
        let to_cache = numb1 + numb2;
        vect.push((numb1, to_cache));
        return (numb1, to_cache);
    };
    //else push it
}

// minus function
fn minus(num1: i32, num2: i32, vect: &mut Vec<(i32, i32)>) -> i32 {
    num1 + num2
}

//this is my first rust project. its a calculator with values cached after each function
//to avoid processing. right now its not a good implementation of it as i would have to
//take the input in a loop to actually cache the value.
//(perosnal project for learning)
