fn main() {

     let islands = vec![
            true,   true,   false,  true,   false,  true, 
            true,   false,  false,  true,   false,  true, 
            false,  false,  false,  false,  false,  true, 
            true,   true,   true,   false,  false,  true, 
            false,  true,   false,  false,  true,   true, 
            false,  true,   true,   false,  false,  true, 
            false,  true,   false,  false,  false,  true, 
            true,   true,   false,  false,  false,  false, 
            true,   true,   false,  false,  false,  false, 
            false,  false,  false,  true,   false,  true, 
            false,  false,  false,  false,  false,  false, 
            false,  false,  false,  false,  false,  false, 
        ];
        
        
        dbg!(num_of_islands(&islands, 6, 12));
}

fn num_of_islands(grid: &[bool], rows: usize, cols: usize) -> usize{
         
        let mut true_values = extract_true_values(&grid, rows);
        let mut num_islands = 0;
        
        while true_values.len() > 0 {
        
        
        
            let mut first = true_values.get(0);
             if let Some(tuple) = first {
             
                let mut row: usize = tuple.0;
                let mut col: usize = tuple.1;
                let mut island = Vec::new();
                island.push((row,col));
                
              
                process_island(&mut island, &mut true_values, &mut num_islands);

            }
            num_islands += 1;
            
        }
        num_islands
        
}
  

fn process_island(
    island: &mut Vec<(usize, usize)>,
    true_values: &mut Vec<(usize, usize)>,
    num_islands: &mut usize,
) {


    if let Some(tuple) = island.get(0) {
        let row = tuple.0;
        let col = tuple.1;
        // Check conditions and modify island and true_values as needed
        if true_values.contains(&(row + 1, col)) {
            island.push((row + 1, col));
            true_values.retain(|&x| !island.contains(&x));
            island.remove(0);
            process_island(island, true_values, num_islands);
        }
        
        if row > 0 && true_values.contains(&(row - 1, col)) {
            island.push((row - 1, col));
            true_values.retain(|&x| !island.contains(&x));
            island.remove(0);
            process_island(island, true_values, num_islands);
        }

        if true_values.contains(&(row, col + 1)) {
            island.push((row, col + 1));
            true_values.retain(|&x| !island.contains(&x));
            island.remove(0);
            process_island(island, true_values, num_islands);
        }
        
        if col > 0 && true_values.contains(&(row, col-1)) {
         
            island.push((row, col - 1));
            true_values.retain(|&x| !island.contains(&x));
            island.remove(0);
            
            process_island(island, true_values, num_islands);
           
        }
        
        true_values.retain(|&x| !island.contains(&x));
        
        // Call the recursive function again
    }
}





fn extract_true_values(grid: &[bool], rows: usize) -> Vec<(usize, usize)> {
    let mut true_values = Vec::new();

    for (index, &value) in grid.iter().enumerate() {
        if value {
            let row = index / rows;
            let col = index % rows;
            true_values.push((row, col));
        }
    }
    true_values
}
