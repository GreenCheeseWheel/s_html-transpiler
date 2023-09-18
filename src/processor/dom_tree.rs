use std::fmt;

#[derive(Debug)]
pub struct DOMTree<T>
{
    value: Option<T>,
    children: Vec<DOMTree<T>>
}

impl<T:Clone> Clone for DOMTree<T>  {
    fn clone(&self) -> Self {
        Self { value: self.value.clone(), children: self.children.clone() }
    }
}

impl<T:fmt::Display + PartialEq + Clone> DOMTree<T> {
    pub fn new(val:T) -> Self
    {
        DOMTree
        {
            value: Some(val),
            children: vec![],
        }
    } 

    
    // THis is basically a bredth-first traversal
    pub fn traverse_and_append(tree: &mut DOMTree<T>, children:&Vec<DOMTree<T>>, target:&T) -> Option<bool>
    {   
        let mut result = None;

        if tree.value.is_some()
        {   


            if DOMTree::append(tree, target, children)
            {
                return Some(true);
            }
            

            for i in 0..tree.children.len()
            {
                result = DOMTree::traverse_and_append(&mut tree.children[i], children, target);

                if result.is_some()
                {
                    break;
                }
            }
        }

        result
    }

    fn append(tree: &mut DOMTree<T>, target:&T, appendage:&Vec<DOMTree<T>>) -> bool
    {
        if *tree.value.as_ref().unwrap() == *target
        {
            let new_val = appendage.get(0).clone().unwrap().clone() ;
            tree.children.push(new_val);
            return true;
        }
        false
    }

}