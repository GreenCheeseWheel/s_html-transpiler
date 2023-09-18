use super::{UNNEST, NEST, dom_tree::DOMTree};

pub fn transpile(text:&str) -> DOMTree<String>
{
    let mut html_file = String::new();
    
    let lines:Vec<&str> = text.lines().collect();

    let mut curr_end_tag:String;

    let mut tree = DOMTree::new("".to_string() );
    
    let mut parent_tag = "".to_string();
    let mut tags_stack:Vec<String> = vec!["".to_string()];
    let mut parents_stack:Vec<String> = vec!["".to_string()];

    for line in lines
    {

        curr_end_tag = String::from(get_tag(&line[1..]));

        curr_end_tag = append_tag( &curr_end_tag, line);
        let curr_end_tag_tree = DOMTree::new(curr_end_tag.clone());

        //println!("Latest tags: {:?}", tags_stack);
        //println!("Parent tags: {:?} \n", parents_stack);

        

        match &line[0..=0] {
            NEST => {
                let latest = tags_stack.pop().unwrap();
                parents_stack.push(latest.clone());
                parent_tag = latest;
            },
            UNNEST => {
                parents_stack.pop();
                parent_tag = parents_stack.pop().unwrap(); 
            },
            _ => parent_tag = parent_tag,
        };

        DOMTree::traverse_and_append(&mut tree, &vec![curr_end_tag_tree], &parent_tag);
        

        tags_stack.push(curr_end_tag);
    }

    tree
}

fn get_tag(line:&str) -> &str
{
    let mut index:usize = 0;
    for (i, byte) in line.as_bytes().iter().enumerate()
    {
        if *byte == b'.' || *byte == b'-'
        {
            index = i;
            break;
        }
    }

    return &line[0..index].trim();
}

fn append_tag(tag: &String, line:&str) -> String
{
    let mut props:Vec<&str> = vec![];

    // We need to check if, after the tag, there are any properties
    let mut index:usize = 0;
    let mut is_prop = false;

    for (i, byte) in line[1..].as_bytes().iter().enumerate()
    {
        
        if *byte == b'"'
        {
            if is_prop 
            {
                is_prop = false;
                continue;
            }

            is_prop = true;
        }

        if *byte == b'.'
        {
            if index > 0 && !is_prop
            {
                props.push(&line[index+1..i]);
                index = i+1;
                continue;
            }
            if !is_prop
            {
                index = i+1;
            }
            
        }

        if *byte == b'-'
        {
                if index == 0
                {   
                    index = i;
                    break;
                }

                if !is_prop
                {
                    props.push(&line[index+1..i]);
                    index = i;
                    break;
                }

                is_prop = !is_prop;
                continue;

        }        

    }

    let mut resulting_tag = String::new();

    resulting_tag.push_str(&format!("<{} ", tag));
    
    for prop in props.iter()
    {
        let property = format!("{} ", *prop);
        
        resulting_tag.push_str(&property);
    }

    resulting_tag.push_str(">");
    resulting_tag.push_str(&line[index+2..]);
    resulting_tag.push_str(&format!("</{}>", tag));

    resulting_tag
}