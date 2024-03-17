use std::{
    collections::HashMap,
    fmt::{self, Display},
    io,
    str::FromStr,
};

use uuid::Uuid;

// Struct to store individual student details
#[derive(Debug, Clone)]
struct Student {
    name: String,
    department: String,
}

impl Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, Department: {}", self.name, self.department)
    }
}

type DB = HashMap<String, Student>;

enum Ctrl {
    Input,
    Exit,
    Display,
}

impl FromStr for Ctrl {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Ctrl, anyhow::Error> {
        let s = s.trim().to_lowercase();
        let s = s.as_str();
        match s {
            "i" => Ok(Ctrl::Input),
            "q" => Ok(Ctrl::Exit),
            "d" => Ok(Ctrl::Display),
            _ => Err(anyhow::anyhow!("Control keyword not recognized")),
        }
    }
}

fn main() {
    let mut students: DB = HashMap::new();

    students.insert(
        Uuid::new_v4().to_string(),
        Student {
            name: "Linus".into(),
            department: "Computer Science".into(),
        },
    );

    println!("Welcome to student record management!");

    println!("{:?}", students);

    println!("Press i to enter student details. \nPress q to exit. \nPress d to display list of students.");

    loop {
        let mut control_str = String::new();

        io::stdin()
            .read_line(&mut control_str)
            .expect("Failed to read line");

        let control: Ctrl = match control_str.parse::<Ctrl>() {
            Ok(ctrl) => ctrl,
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
        };

        match control {
            Ctrl::Exit => {
                println!("Thank you!");
                break;
            }
            Ctrl::Display => {
                for (id, student) in students.iter() {
                    println!("id: {} {}", id, student);
                }
            }
            Ctrl::Input => {
                let uuid = Uuid::new_v4().to_string();
                match students.insert(uuid.clone(), get_details()) {
                    Some(updated) => println!("Updated student details: {}", updated),
                    None => {
                        println!(
                            "Added student: id: {} {}",
                            uuid,
                            students.get(&uuid).expect("can't fail")
                        );
                    }
                }
            }
            _ => {
                println!("Invalid input.");
            }
        }
    }
}

fn get_details() -> Student {
    println!("Please enter the name:");
    let mut data = String::new();

    io::stdin()
        .read_line(&mut data)
        .expect("Failed to read line");

    let name = data.trim().to_string();

    println!("Please enter the Department:");

    data.clear();
    io::stdin()
        .read_line(&mut data)
        .expect("Failed to read line");

    let department = data.trim().to_string();

    Student { name, department }
}
