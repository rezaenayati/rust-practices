fn main() {
    const ARRAY: [&str; 12] = [
        "Drummers Drumming",
        "Pipers Piping",
        "Lords a Leaping",
        "Ladies Dancing",
        "Maids a Milking",
        "Swans a Swimming",
        "Geese a Laying",
        "Golden Rings",
        "Calling Birds",
        "French Hens",
        "Turtle Doves",
        "a Partridge in a Pear Tree"
    ];

    const FIRST_LINE: &str = "On the first day of Christmas";
    const SECOND_LINE: &str = "my true love sent to me:";

    println!("=======================================================================");
    println!("Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”");

    println!("starts in ... \n\n");



    for index in (1..13).into_iter() {
        println!("{}", FIRST_LINE);
        println!("{}", SECOND_LINE);    
        for jindex in (0..index).rev() {
            let mut nu: &str = &(jindex + 1).to_string();
            if jindex == 0 {
                nu = "And"
            }

            println!("{} {}", nu, ARRAY[11-jindex]);
        }

        println!("\n\n");    

    }
    

    println!("=======================================================================");

}