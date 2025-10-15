use std::error::Error;
use std::io;
use std::io::Write;
use crate::sevices::*;
use crate::models::*;

pub fn handle_list() -> Result<(), Box<dyn Error>> {
    let students = read_from_text()?;
    list_students(&students);
    Ok(())
}

pub fn handle_add(id: &str, name: &str, score: i32) -> Result<(), Box<dyn Error>> {
    let mut students = read_from_text()?;
    let student = Student {
        id: id.to_string(),
        name: name.to_string(),
        score: score,
    };
    if let Some(_exist) = students
        .iter()
        .enumerate()
        .find(|(_, s)| s.id == student.id)
        {
            println!("Student has been added.");

        }
    else {
        students.push(student);
        write_to_text(&students)?;
        println!("Student added.");
    }
    Ok(())
}

pub fn handle_delete(id: &str) -> Result<(), Box<dyn Error>> {
    let students = read_from_text()?;
    if let Some(student) = students
        .iter()
        .enumerate()
        .find(|(_, s)| s.id == id)
        .map(|(_, s)| s.clone())
    {
        let left_students = students
            .into_iter()
            .filter(|m| *m != student)
            .collect::<Vec<Student>>();
        write_to_text(&left_students)?;
        println!("Student deleted.");
    }
    Ok(())
}

pub fn handle_edit(id: &str) -> Result<(), Box<dyn Error>> {
    let mut students = read_from_text()?;
    if let Some(student) = students
        .iter_mut()
        .enumerate()
        .find(|(_, s)| s.id == id) 
        .map(|(_, s)| s)
    {
        print!("Enter the new name:");
        io::stdout().flush()?;
        let mut name = String::new();
        io::stdin().read_line(&mut name)?;
        let name = name.trim();
        if !name.is_empty() {
            student.name = name.to_string();
        } else {
            println!("Name cannot be empty.");
            return Ok(());
        }

        print!("Enter the new score:");
        io::stdout().flush()?;
        let mut score = String::new();
        io::stdin().read_line(&mut score)?;
        let score = score.trim();
        if let Ok(score) = score.parse::<i32>() {
            student.score = score;
        } else {
            println!("Invalid score.");
            return Ok(());
        }
    }

    write_to_text(&students)?;
    println!("Student edited.");
    Ok(())
}