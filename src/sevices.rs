use std::error::Error;
use std::fs;
use std::io::{ErrorKind,BufReader, BufRead, Write};
use crate::models::*;
use unicode_width::UnicodeWidthStr;

pub fn read_from_text() -> Result<Vec<Student>, Box<dyn Error>> {
    let path = "Students.txt";
    let mut students = Vec::new();

    match fs::metadata(path){
        Ok(_) => {
            let file = fs::File::open("Students.txt")?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                let parts: Vec<&str> = line.split(' ').collect();
                if parts.len() != 3 {
                    continue;
                }
                let student = Student {
                    id: parts[0].to_string(),
                    name: parts[1].to_string(),
                    score: parts[2].parse().unwrap_or(0),
                };
                students.push(student);
            }

            Ok(students)
        }
        Err(err) =>{
            if err.kind() == ErrorKind::NotFound {
                Ok(students)
            } else {
                println!("{err}");
                panic!()
            }
        }
    }
    
    
}

pub fn write_to_text(students: &[Student]) -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::create("Students.txt")?;
    for student in students {
        writeln!(file, "{} {} {}", student.id, student.name, student.score)?;
    }
    Ok(())
}

pub fn list_students(students: &[Student]) {
    println!("-------------------------------------------------------------------------------");
    println!("|    学号    |    姓名    |    成绩    |    学号    |    姓名    |    成绩    |");
    let mut flag = false;
    students.iter().for_each(|m | {
        if flag == false {
            println!("-------------------------------------------------------------------------------");
        }
        let name = pad_display_width(&m.name, 12);
        print!("|{:^12}|{}|{:^12}", m.id, name, m.score);
        if flag == true {
            print!("|\n");
        }
        flag = !flag;
    });
    if flag == true {
        println!("|");
        println!("----------------------------------------");
    }
    else {
        println!("-------------------------------------------------------------------------------");
    }
}

fn pad_display_width(s: &str, target_width: usize) -> String {
    let width = UnicodeWidthStr::width(s);
    if width >= target_width {
        return s.to_string();
    }
    let total_pad = target_width - width;
    let left_pad = total_pad / 2;
    let right_pad = total_pad - left_pad;
    format!("{}{}{}", " ".repeat(left_pad), s, " ".repeat(right_pad))
}