#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list==second_list{
        Comparison::Equal
    } else if first_list.is_empty() || (first_list.len()<second_list.len()&&second_list.windows(first_list.len()).any(|x| x==first_list)){
        Comparison::Sublist
    } else if second_list.is_empty() || (first_list.len()>second_list.len()&&first_list.windows(second_list.len()).any(|x|x==second_list)){
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
