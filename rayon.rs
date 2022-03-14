 use rayon::prelude::*;

 // sequential 
 let ids = vec![ "2345", "877865" ,"5678  ", "-908" , "twelve" , "1001  ", "  999"];

 let ids = ids
    .iter()
    .map(|id| id.trim())
    .filter_map(|id| id.parse().ok())
    .filter(|num| num >= &1000)
    .collect::<Vec<usize>>();


    // parallel
let ids = vec![ "2345", "877865" ,"5678  ", "-908" , "twelve" , "1001  ", "  999"];

 let ids = ids
    .per_iter()
    .map(|id| id.trim())
    .filter_map(|id| id.parse().ok())
    .filter(|num| num >= &1000)
    .collect::<Vec<usize>>();


// Sorting
let mut ids: Vec<usize> = ids;
ids.par_sort();
for id in ids {
    println!("{}", id);
}


// for .. in (Error)

let ids = vec![ "2345", "877865" ,"5678  ", "-908" , "twelve" , "1001  ", "  999"];

for id in ids.par_iter() {
     println!("{}", id);
    }

----> Error <-----

// for Each (Correct)
let ids = vec![ "2345", "877865" ,"5678  ", "-908" , "twelve" , "1001  ", "  999"];


ids.par_iter()
  .for_each(|id| println!("{}" , id));