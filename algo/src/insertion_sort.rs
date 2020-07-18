pub mod insertion {
    fn sort(v: &mut Vec<i32>) {
        // This is an implementation of an ascending insertion sort algo
        // This takes a mutable reference to arr
        // since it needs to modify it in place, and not return a new vec
        // if this was just `mut Vec<i32>` then it would not change the
        // vec that is in main.rs
        // The original thought process:
        // Insertion sort:
        // 1. Take first elemet
        // 2. Compare it to all past elements
        // 3. Place ahead of the first element that is smaller than it
        // 4. If no smaller elements then place at front
        for selected in 0..v.len() {
            let mut past = selected;
            while past > 0 {
                if v[past] < v[past - 1] {
                    let temp = v[past - 1];
                    v[past - 1] = v[past];
                    v[past] = temp;
                }
                past -= 1;
            }
        }
    }
    pub fn test_sort() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut arr: Vec<i32> = Vec::new();
        for _ in 0..10000{
            arr.push(rng.gen_range(0, 100));
        }
        println!("{:?}", arr);
        sort(&mut arr);
        println!("{:?}", arr);
    }
}
