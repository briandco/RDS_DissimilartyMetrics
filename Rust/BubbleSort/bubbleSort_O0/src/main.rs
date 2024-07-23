fn main() {

    let mut arr = [10,60,40,90,30,80,50,10,70,10,60,40,90,30,80,50,10,70];
    println!("Original array: {:?}", arr);

    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
    
}

fn bubble_sort(arr:&mut [i32]){
    let n = arr.len();

    for i in 0..n{
        for j in 0..(n-i-1){
            if arr[j] > arr[j+1]{
                arr.swap(j, j+1);
            }
        }
    }
}
