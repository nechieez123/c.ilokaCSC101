fn main() {
    // Create an empty vector "City"
    let mut city : Vec<String> = Vec::new();
    // Print City Vector
    println!("The City vector has element {}",city.len());
    // Push new elements into
    let mut input1 = String::new();
    println!("How many Cities do you want enter");
    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    let new_city:String = input2.trim().parse().expect("Invalid input");
    city.push(new_city);
}
print!("Your preferred cities are:\n");
let mut count=1;
//loop to iterate elements in vector
for i in new_city
{
    // iterating through i on the vector
    println!("{} {}",cout, i);
    count+=1;
}



