fn main() {
    /* Two ways to initiate a vector, two ways to get data, one with Option<T>.
    let mut v: Vec<i32> = Vec::new();
    let n = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    // Nope, crash.
    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    // Cannot push data to a borrowed variable, error.
    let mut y = vec![1, 2, 3, 4, 5];
    let first = &y[0];
    y.push(6);*/

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}
