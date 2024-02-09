

fn main (){
    let mut a_box = Box::new(17);
    let mut b_box = a_box();

    println!("{}", *a_box);
    println!("{}", *b_box);

    *a_box = 18; 


    println!("{}", *a_box);
}