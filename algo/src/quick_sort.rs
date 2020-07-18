pub mod quick {
    fn _partition(arr: &mut Vec<i32>, lo: usize, hi: usize) -> usize {
        let pivot = arr[(lo + hi) / 2];
        println!(
            "Starting Partition = Pivot: {}, Lo: {}, Hi: {}, Arr: {:?}",
            pivot, lo, hi, arr
        );
        let mut i = lo;
        let mut j = hi - 1;
        //         println!("{:?}", arr);

        loop {
            //             if i >= j {
            //                 return j;
            //             }
            if arr[i] == arr[j] {
                return j;
            }
            while arr[i] < pivot {
                //                 if arr[i] == pivot {
                //                     break;
                //                 }
                println!("I: {}, arr[i]: {}, arr : {:?}", i, arr[i], arr);
                i += 1;
            }
            while arr[j] > pivot {
                println!("j: {}, arr[j]: {}", j, arr[j]);
                j -= 1;
            }
            if i >= j {
                return j;
            }
            println!(
                "Swapping  arr[i]: {} with arr[j]: {} Pivot: {}",
                arr[i], arr[j], pivot
            );
            let temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
        }
    }
    fn _sort(arr: &mut Vec<i32>, lo: usize, hi: usize) {
        println!("SORT: LO {} HIGH {}", lo, hi);
        if lo < hi {
            let p = _partition(arr, lo, hi);
            _sort(arr, lo, p);
            _sort(arr, p + 1, hi);
        }
    }

    pub fn test_sort() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut arr: Vec<i32> = Vec::new();
        for _ in 0..500 {
            arr.push(rng.gen_range(0, 100));
        }
        //         let mut arr = vec![1,1,6,6,6,1,7, 7, 8,  5, 4, 9, 6, 3, 5, 5, 5];
        println!("{:?}", arr);
        let l = arr.len().clone();
        _sort(&mut arr, 0, l);
        println!("{:?}", arr);
    }
}
