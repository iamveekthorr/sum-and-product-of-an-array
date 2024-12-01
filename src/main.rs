fn main() {
    let arr = vec![1, 2, 3, 4, 5];

    let result = sum_of_array(&arr);
    let product = product_of_array(&arr);

    println!("The sum is = {}. The product is = {}", result, product);
}

fn sum_of_array(arr: &Vec<i32>) -> i32 {
    if arr.len() == 0 {
        return 0;
    }

    let mut sum = 0;

    for &i in arr {
        sum = sum + i;
    }

    sum
}

fn product_of_array(arr: &Vec<i32>) -> i32 {
    if arr.len() == 0 {
        return 0;
    }

    let mut product = 1;

    for &i in arr {
        product = product * i;
    }

    product
}
