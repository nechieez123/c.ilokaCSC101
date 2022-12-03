fn main() {
    let name= "Pan-Atlantic University";
    let uni:&str ="Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    let addr:&str ="Nigeria";
    println!("Name:{}", name );
    println!("University: {}, \nAddress: {}",uni,addr );

    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Technology";
    println!("Department: {}, \nSchool: {}",department,school );
}