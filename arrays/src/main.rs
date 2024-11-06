fn main() {
    let x : [i32; 5] = [1,2,3,4,5];
    let mut index = 0;
    loop
    {
        println!("{}", x[index]);
        index += 1;
        if index == 5
        {
            break;
        }
    }
}
