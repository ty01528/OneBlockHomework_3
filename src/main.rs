
// 固定类型（比如i32）的数组排序
fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();

    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                // 交换arr[j]和arr[j+1]
                arr.swap(j, j + 1);
            }
        }
    }
}
// 使用泛型实现的对任意类型的排序
fn bubble_sort2<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();

    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                // 交换arr[j]和arr[j+1]
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    println!("固定类型（比如i32）的数组排序");
    let mut arr = [5,12,3,1,4];
    println!("原数组为:{:?}", arr);
    bubble_sort(&mut arr);
    println!("排序结果:{:?}", arr);
    println!("使用泛型实现的对任意类型的排序");
    let mut arr = [  "cherry", "apple","date","banana"];
    println!("原数组为:{:?}", arr);
    bubble_sort2(&mut arr);
    println!("排序结果:{:?}", arr);
}
