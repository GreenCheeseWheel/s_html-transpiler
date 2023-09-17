pub fn transpile(text:&str) -> String
{
    let mut html_file = String::new();
    
    let lines:Vec<&str> = text.lines().collect();

    let mut curr_end_tag:String;

    for line in lines
    {
        curr_end_tag = String::from(get_tag(&line[1..]));
        curr_end_tag = append_tag(&mut html_file, &curr_end_tag, line);
        
        html_file.push_str(format!("{}\n", curr_end_tag).as_str() );
    }

    html_file
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

fn append_tag(html_file: &mut String, tag: &String, line:&str) -> String
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

    html_file.push_str(&format!("<{} ", tag));

    for prop in props.iter()
    {
        let property = format!("{} ", *prop);
        html_file.push_str(&property);
    }

    html_file.push_str(">");
    html_file.push_str(&line[index+2..]);
    
    format!("</{}>", tag)
}