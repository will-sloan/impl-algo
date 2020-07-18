pub mod quick {
    fn _partition(arr: Vec<i32>) -> Vec<i32> {
    }
    fn _sort(arr: Vec<i32>) -> Vec<i32> {

    }

    pub fn test_sort() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut arr: Vec<i32> = Vec::new();
        for _ in 0..10000 {
            arr.push(rng.gen_range(0, 100));
        }
        println!("{:?}", arr);
        sort(arr);
        println!("{:?}", arr);
    }
}
