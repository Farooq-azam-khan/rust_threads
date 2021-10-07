use rayon::prelude::*;

fn main() {
    let arr = &[1,25,-4,10];
    let max = find_max(arr);
    assert_eq!(max, Some(25));
    println!("Found the max {:?}", max);    
    
    let v: Vec<Person> = vec![
        Person {age: 23}, 
        Person {age: 19}, 
        Person {age: 42}, 
        Person {age: 17}, 
        Person {age: 17}, 
        Person {age: 31}, 
        Person {age: 30}
    ]; 

    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32; 
    let sum_over_30 = v.par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30) 
        .reduce(|| 0, |x,y| x+y); 

    let alt_sum_30: u32 = v.par_iter() 
        .map(|x| x.age)
        .filter(|&x| x>30)
        .sum(); 
    
        let avg_over_30 = sum_over_30 as f32 / num_over_30; 
        let alt_avg_over_30 = alt_sum_30 as f32 / num_over_30; 

        assert!((avg_over_30 - alt_avg_over_30).abs() < std::f32::EPSILON); 
        println!("The average age of people oler than 30 is {}", avg_over_30); 

}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THREASHOLD: usize = 2; 
    if arr.len() <= THREASHOLD {
        return arr.iter().cloned().max(); 
    }

    let mid = arr.len() / 2; 
    let (left, right) = arr.split_at(mid); 

    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left)); 
        let thread_r = s.spawn(|_| find_max(right)); 

        let max_l = thread_l.join().unwrap()?; 
        let max_r = thread_r.join().unwrap()?; 

        Some(max_l.max(max_r))
    }).unwrap() 
}


struct Person {
    age: u32
}

