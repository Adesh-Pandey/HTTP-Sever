//  8 byte read at a time from the file
// fn read_file() -> io::Result<()> {
//     let path = Path::new("content/message.txt");
//     let file = File::open(path)?;
//     let mut reader = BufReader::new(file);
//     let mut buffer = [0u8; 8];
//
//     while reader.read(&mut buffer)? != 0 {
//         buffer.iter().for_each(|b| print!("{}", *b as char));
//         println!("\nFinished one set of 8 byte")
//     }
//
//     println!("Finished");
//
//     Ok(())
// }

// read line
// fn read_file_2() -> io::Result<()> {
//     let path = Path::new("content/message.txt");
//     let file = File::open(path)?;
//     let mut reader = BufReader::new(file);
//     let mut str = String::new();
//     while reader.read_line(&mut str)? != 0 {
//         println!("{}", str);
//         str = String::new();
//     }
//     Ok(())
// }
//
fn main() {}
