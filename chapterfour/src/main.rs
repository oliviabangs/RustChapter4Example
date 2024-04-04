#[derive(Debug)]
struct Table(Vec<Vec<String>>);

impl From<&str> for Table {
    fn from(csv: &str) -> Self {
        let mut resulting_table: Vec<Vec<String>> = Vec::new();

        //Turns each row into its own string and iterates through them
        let rows: Vec<&str> = csv.split("\n").collect();
        for i in 0..rows.len() {
            let mut columns = Vec::new();

            //Splitting up each row into its individual elements 
            //And iterating through them in order to make them into "String" before putting them into the Vec
            let elems: Vec<&str> = rows[i].split(",").collect();
            for j in 0..elems.len() {
                columns.push(elems[j].into());
            }
            resulting_table.push(columns);
        }

        Table(resulting_table)
    }
}

impl From<Table> for String {
    fn from(table: Table) -> Self {
        let mut rows: Vec<String> = Vec::new();

        //Going through each inner Vec and joining each of their elements together into a String
        for i in 0..table.0.len() {
            rows.push(table.0[i].join(","));
        }

        //Joining the resulting Strings from each inner Vec together
        let resulting_string: String = rows.join("\n");
        resulting_string
    }
}

fn main() {
    let csv_string: &str = "First Name,Last Name,Phone Number,Email Address\nAda,Lovelace,123-456-7890,analytical@gmail.com\nGrace,Hopper,987-654-3210,cobol@yahoo.com";

    //Showing that implementing the From gets you the Into
    let csv_table_two: Table = Table::from(csv_string);
    let csv_table_one: Table = csv_string.into();

    //Printing both to show that they do the same thing
    println!("{:?}", csv_table_one);
    println!("{:?}", csv_table_two);

    //Converting in the other direction (but as a String and not a &str this time)
    let back_to_string_two: String = String::from(csv_table_two);
    let back_to_string_one: String = csv_table_one.into();

    //Printing both to show that it turns it back into the original
    println!("{}", back_to_string_one);
    println!("{}", back_to_string_two);
}
