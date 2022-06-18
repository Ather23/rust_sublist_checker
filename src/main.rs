use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn vec_from_hashset(vec: &Vec<i32>) -> HashSet<i32>{
    let mut resultVec = vec.clone();
    resultVec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // resultVec.reverse();
    HashSet::from_iter(resultVec.iter().cloned())
}

fn sublist_checker(list_a:&Vec<i32>,list_b:&Vec<i32>)->Comparison{

    let mut a = list_a;
    let mut b = list_b;
    let hashA = vec_from_hashset(&mut a);
    let hashB = vec_from_hashset(&mut b);

    let size_a = hashA.len();
    let size_b = hashB.len();

    let mut max_vec:Option<HashSet<i32>> =None;
    let mut min_vec:Option<HashSet<i32>> =None;

    if size_a > size_b{
        max_vec = Some(hashA);
        min_vec = Some(hashB);
    }
    else
    {
        max_vec = Some(hashA);
        min_vec = Some(hashB);
    }

    println!("Max vec {:?}",max_vec.as_ref().unwrap());
    println!("Min vec {:?}",min_vec.as_ref().unwrap());


    let mut result:Comparison = Comparison::Unequal;
    let mut matchCounter = 0;
    for mx in max_vec.as_ref().unwrap().iter(){
        for mn in min_vec.as_ref().unwrap().iter(){
            if mx == mn{
                matchCounter+=1;
            }
        }
    }

    println!("Match counter {:?}",matchCounter);
    if size_a == size_b 
    {
        if matchCounter == size_a{
            result = Comparison::Equal;
        }
    }
    else
    {
        if matchCounter >0{
            result = Comparison::Sublist;
        }
        else
        {
            result = Comparison::Unequal;
        }
    }
    return result;    
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_is_sublist_of_b() {
        let mut a = vec![4,3,2,1];
        let mut b = vec![1,2,3];
        let result = sublist_checker(&a,&b);      
        assert_eq!(result,Comparison::Sublist);
    }

    #[test]
    fn a_is_not_sublist_of_b() {
        let mut a = vec![4,3,2,1];
        let mut b = vec![10];
        let result = sublist_checker(&a,&b);      
        assert_eq!(result,Comparison::Unequal);
    }

    #[test]
    fn a_is_equal_to_b() {
        let mut a = vec![4];
        let mut b = vec![4];
        let result = sublist_checker(&a,&b);      
        assert_eq!(result,Comparison::Equal);
    }


    #[test]
    fn a_is_sublist_of_b_when_b_is_larger() {
        let mut a = vec![4];
        let mut b = vec![4,1,2,3,7];
        let result = sublist_checker(&a,&b);      
        assert_eq!(result,Comparison::Sublist);
    }

}
