use super::{ANCHOR, P};

pub fn transpile(text:&str) -> String
{
    let mut html_file = String::new();
    
    let lines:Vec<&str> = text.lines().collect();

    let mut curr_end_tag=String::new();

    for line in lines
    {
        curr_end_tag = append_tag(&mut html_file, line.as_bytes()[1], line);
        
        html_file.push_str(format!("{}\n", curr_end_tag).as_str() );
    }

    html_file
}

fn append_tag(html_file: &mut String, tag: u8, line:&str) -> String
{
    let mut props:Vec<&str> = vec![];

    // We need to check if, after index 1, there are any properties for the tag
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

    html_file.push_str(&format!("<{} ", tag as char));

    for prop in props.iter()
    {
        let property = format!("{} ", *prop);
        html_file.push_str(&property);
    }

    html_file.push_str(">");
    html_file.push_str(&line[index+2..]);

    println!("Las props: {:?}", props);
    format!("</{}>", tag as char)
}