fn find_slice_indices(s: &String, slice: &str) -> Option<(usize, usize)> {
    // Get raw pointers to the start of the String and the slice
    let s_ptr = s.as_ptr() as usize;
    let slice_ptr = slice.as_ptr() as usize;

    // Ensure the slice is actually part of the String
    if slice_ptr < s_ptr || slice_ptr + slice.len() > s_ptr + s.len() {
        return None;
    }

    // Calculate the starting index
    let start_index = slice_ptr - s_ptr;

    // Calculate the ending index
    let end_index = start_index + slice.len();

    Some((start_index, end_index))
}