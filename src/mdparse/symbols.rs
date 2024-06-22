///Used at the beginning of a line to declare a header
#[macro_export]
macro_rules! title_marker{
    () => { '#' }
}

///Used at the beginning of a line to declare an unordered list
#[macro_export]
macro_rules! unordered_list_marker {
    () => {'-'};
}

#[macro_export]
macro_rules! ordered_list_ender {
    () => { '.' }
}

#[macro_export]
macro_rules! digit {
    () => { '0' ..= '9'};
}

#[macro_export]
macro_rules! whitespace {
    () => {' ' | '\t' };
}
