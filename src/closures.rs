

pub fn test_closure(x:i16, y:i16) -> i16{
    
    let add_num= |x, y| x+y;

    let result = add_num(x,y);
    return result;
}

