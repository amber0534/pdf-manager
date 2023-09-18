/* @File:       main.rs
 * @Author:     amber
 * @Date:       18/09/23
 * @Abstract:   CLI om pdfs te beheren
 */

//********************************************************************

use std::env;
use std::process::exit;

// For file handling
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

//use std::io::Write;

// For errors
// use std::error::Error;

const FILENAME_PDFS: &str =
    "/home/amber/docs/scripts/rofi/pdf_opener/pdf_manager/pdfs.txt";

//********************************************************************

// Error struct voor fouten bij --delete
#[derive(Debug)]
enum DelPdfError {
    ArgumentsError,        // Not enough arguments
    FileError(String),     // File not found
    AbsoluteError(String), // Couldn't convert file to abolute path
    StorageError(String),  // Error looking up in pdfs.txt
}

impl std::error::Error for DelPdfError {}

impl std::fmt::Display for DelPdfError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            DelPdfError::ArgumentsError => {
                writeln!(f, "Error adding PDF: Not enough arguments provided.")?;
                write!(f, "Usage: `pdfmgr --del <file>`")
            }
            DelPdfError::FileError(s) => {
                write!(
                    f,
                    "Error deleting PDF from list: File `{}` not found.",
                    s
                )
            }
            DelPdfError::AbsoluteError(s) => {
                write!(
                    f,
                    "Couldn't convert `{}` to an absolute path.",
                    s
                )
            }
            DelPdfError::StorageError(s) => {
                write!(f, "Error parsing `{}` in database.", s)
            }
        }
    }
}

fn delete_pdf(
    args: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 3 {
        return Err(Box::new(DelPdfError::ArgumentsError));
    }
    // Check if file exists
    if !Path::new(&args[2]).exists() {
        return Err(Box::new(AddPdfError::FileError(
            args[2].clone(),
        )));
    }

    let absolute_filename = match fs::canonicalize(&args[2])?
        .into_os_string()
        .into_string()
    {
        Ok(s) => s,
        Err(_) => {
            return Err(Box::new(DelPdfError::AbsoluteError(
                args[2].clone(),
            )));
        }
    };

    // Check if file is in pdfs.txt
    let mut line_vector: Vec<String> =
        fs::read_to_string(FILENAME_PDFS)?
            .split("\n")
            .map(|s| s.to_string())
            .collect();
    let mut index: i32 = -1;
    for i in 0..line_vector.len() as i32 {
        let line = &line_vector[i as usize];
        let first_word = line.split(" @ ").next().unwrap_or("");
        if first_word == "" {
            return Err(Box::new(DelPdfError::StorageError(
                line.clone(),
            )));
        }

        if first_word == absolute_filename {
            index = i;
            break;
        }
    }

    // Remove index from line_vector
    if index == -1 {
        return Err(Box::new(DelPdfError::FileError(
            absolute_filename,
        )));
    }
    line_vector.remove(index as usize);

    // Re-write updated pdfs.txt
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILENAME_PDFS)?;
    for line in line_vector {
        if line != "" {
            file.write_all(line.as_bytes())?;
            file.write_all(b"\n")?; // Add newline
        }
    }

    Ok(())
}

//********************************************************************

// Error enum voor fouten bij --add
#[derive(Debug)]
enum AddPdfError {
    ArgumentsError,        // Not enough arguments
    FileError(String),     // File not found
    DescriptionError,      // Description contains a '@'
    AbsoluteError(String), // For error with absolute path
}

impl std::error::Error for AddPdfError {}

impl std::fmt::Display for AddPdfError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            AddPdfError::ArgumentsError => {
                writeln!(f, "Error adding PDF: Not enough arguments provided.")?;
                write!(
                    f,
                    "Usage: `pdfmgr --add <file> <description>`"
                )
            }
            AddPdfError::FileError(s) => {
                write!(
                    f,
                    "Error adding PDF to list: File `{}` not found.",
                    s
                )
            }
            AddPdfError::DescriptionError => {
                write!(f, "Error adding PDF to list: ")?;
                write!(
                    f,
                    "Description may not contain the character `@`"
                )
            }
            AddPdfError::AbsoluteError(s) => {
                write!(
                    f,
                    "Couldn't convert `{}` to an absolute path.",
                    s
                )
            }
        }
    }
}

//********************************************************************
// Schrijf weg naar

fn add_pdf(
    args: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Args: <script> < --add> <filename> <description>
    if args.len() < 4 {
        return Err(Box::new(AddPdfError::ArgumentsError));
    }

    if !Path::new(&args[2]).exists() {
        return Err(Box::new(AddPdfError::FileError(
            args[2].clone(),
        )));
    }

    // Loop over description om te zorgen dat er geen @ in zit
    for i in 3..args.len() {
        if args[i].contains("@") {
            return Err(Box::new(AddPdfError::DescriptionError));
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(FILENAME_PDFS)?;

    let absolute_filename = match fs::canonicalize(&args[2])?
        .into_os_string()
        .into_string()
    {
        Ok(s) => s,
        Err(_) => {
            return Err(Box::new(AddPdfError::AbsoluteError(
                args[2].clone(),
            )));
        }
    };

    write!(file, "{} @", absolute_filename)?;
    for i in 3..args.len() {
        write!(file, " {}", &args[i])?;
    }
    writeln!(file, "")?;

    Ok(())
}

//********************************************************************

// Error struct voor formatting errors bij de -l optie
#[derive(Debug)]
struct FormatError(String);

impl std::error::Error for FormatError {}

impl std::fmt::Display for FormatError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Kan format optie bij -l niet parsen: `{}` is geen bekende optie.", self.0)
    }
}

//********************************************************************

fn list_pdfs(
    args: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let line_vector: Vec<String> = fs::read_to_string(FILENAME_PDFS)?
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let mut format_str1: &str = "path";
    let mut format_str2: &str = "description";

    if args.len() >= 3 {
        format_str2 = "";
        format_str1 = match &args[2] as &str {
            "d" | "description" => "description",
            "p" | "path" => "path",
            &_ => {
                return Err(Box::new(FormatError(args[2].clone())));
            }
        };
    }
    if args.len() >= 4 {
        format_str2 = match &args[3] as &str {
            "d" | "description" | "desc" => "description",
            "p" | "path" | "file" | "f" => "path",
            &_ => {
                return Err(Box::new(FormatError(args[3].clone())));
            }
        };
    }

    println!("Format: <{}> @ <{}>", format_str1, format_str2);
    println!("========================================");
    for line in line_vector {
        let split_line: Vec<String> =
            line.split(" @ ").map(|s| s.to_string()).collect();
        if split_line.len() != 2 {
            continue;
        }

        match format_str1 {
            "path" => print!("{}", split_line[0]),
            "description" => print!("{}", split_line[1]),
            &_ => {}
        }
        match format_str2 {
            "path" => print!(" @ {}", split_line[0]),
            "description" => print!(" @ {}", split_line[1]),
            &_ => {}
        }
        println!();
    }
    Ok(())
}

//********************************************************************

fn print_help() -> Result<(), Box<dyn std::error::Error>> {
    println!("pdfmgr:");
    println!("Een CLI voor het managen van de pdfs waar het rofi");
    println!("script `pdf_opener` toegang tot heeft.");
    println!("");
    println!("Gebruik: `pdfmgr <options>`");
    println!("");
    println!("Mogelijke options");
    println!("=====================================================");
    println!("-h, --help");
    println!("  Print deze hulp");
    println!("-a, --add <file> <desc>");
    println!("  Voeg <file> toe met de beschrijving <desc> aan de");
    println!("  database met pdf bestanden. Het pad mag relatief");
    println!("  zijn.");
    println!("-l, --list [fmt_1] [fmt_2]");
    println!("  Print alle opgeslagen pdfs en hun beschrijvingen.");
    println!("  fmt_1 en fmt_2 kunnen specificeren wat er precies");
    println!("  geprint wordt. Toegestane waarden:");
    println!("    `p`,`path`,`f`,`file`: Het path naar het bestand");
    println!("    `d`,`desc`,`description`: De beschrijving");
    println!("-d, --del <file>");
    println!("  Verwijder <file> uit de database. Het pad mag");
    println!("  relatief zijn.");
    Ok(())
}

//********************************************************************

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        match print_help() {
            Ok(_) => {}
            Err(e) => println!("Error! {e}"),
        }
        exit(1);
    }

    let result = match &args[1] as &str {
        "-h" | "--help" => print_help(),
        "-l" | "--list" => list_pdfs(&args),
        "-a" | "--add" => add_pdf(&args),
        "-d" | "--delete" => delete_pdf(&args),
        &_ => print_help(),
    };

    match result {
        Ok(_) => {}
        Err(e) => eprintln!("{e}"),
    }
}

//********************************************************************
