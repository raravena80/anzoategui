fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let vec3 = vec![7, 8, 9];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));
    println!("9 in vec3: {}", vec3.into_iter().any(|x| x == 9));


    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    let array3 = [7, 8, 9];

    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`.
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
    println!("9 in array3: {}", array3.into_iter().any(|&x| x == 9));
}
