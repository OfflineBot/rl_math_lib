#[allow(unused)]
pub fn divmod(left: usize, right: usize) -> (usize, usize) {
    let div = left / right;
    let modulo = left % right;
    (div, modulo)
}


#[allow(unused)]
pub fn argmax<T>(array: &Vec<Vec<T>>, index: usize) -> usize
where
    T: PartialOrd + Clone
{
    let arr_slice = &array[index];
    let mut max_index = 0;
    let mut highest_num = arr_slice[0].clone();
    arr_slice
        .iter()
        .enumerate()
        .map(|(i, item)| {
            if &highest_num < item {
                highest_num = item.clone();
                max_index = i;
            }
        });
    max_index
}

#[allow(unused)]
pub fn max<T>(array: &Vec<Vec<T>>, index: usize) -> T
where
    T: PartialOrd + Clone
{
    let arr_slice = &array[index];
    let mut max_num = arr_slice[0].clone();
    arr_slice
        .iter()
        .map(|item| {
            if item > &max_num {
                max_num = item.clone();
            }
        });
    max_num
}

#[allow(unused)]
pub fn find(array: &Vec<Vec<char>>, item: char) -> Result<(usize, usize), String> {
    let pos: Option<(usize, usize)> = array
        .iter()
        .enumerate()
        .find_map(|(i , row)| {
            row
                .iter()
                .enumerate()
                .find_map(|(j, col)| {
                    if col == &item {
                        return Some((i, j));
                    } else {
                        None
                    }
                })
            });
    let final_pos = match pos {
        Some(final_pos) => final_pos,
        None => return Err(String::from("Item not find in given array!")),
    };
    Ok(final_pos)
}