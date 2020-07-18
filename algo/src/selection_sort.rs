pub mod selection {
    fn sort(arr: &mut Vec<i32>) {
        let mut v: Vec<i32> = Vec::new();
        for _ in 0..arr.len() {
            let mut min: usize = 0;
            for j in 0..arr.len() {
                if arr[j] < arr[min] {
                    //                     println!("got one");
                    min = j;
                }
            }
            //             println!("Pushed");
            v.push(arr.remove(min));
        }
        *arr = v;
    }

    pub fn test_sort() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut arr: Vec<i32> = Vec::new();
        for _ in 0..10000 {
            arr.push(rng.gen_range(0, 100));
        }
        println!("{:?}", arr);
        sort(&mut arr);
        println!("{:?}", arr);
    }
}
