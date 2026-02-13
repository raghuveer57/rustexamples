fn main() {
    let mut counter:u32 = 1;
    
    let result = loop{
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    check_loops_withn_loops();
}

// 
fn check_loops_withn_loops(){
    let mut inner_count = 1;
    let mut outer_count = 1;

    'outer_loop: loop {
        println!("Outer loop count: {}", outer_count);
        outer_count += 1;

        'inner_loop: loop {
            println!("Inner loop count: {}", inner_count);
            inner_count += 1;

            if inner_count == 5 {
                inner_count = 1; // reset inner_count
                break 'inner_loop;
            }
        }

        if outer_count == 10 {
            break 'outer_loop;
        }
    }
}