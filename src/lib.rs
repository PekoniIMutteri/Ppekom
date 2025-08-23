use pimage::{Color, Pimage};
use std::fs;
use std::io::Error;

/// A shorthand for `Err(Error::other(string))`
fn error<T>(string: &str) -> Result<T, Error> {
    Err(Error::other(string))
}

/// Tries to reads a PNM P6 (binary PPM) from the given file, and returns that PPM.
///
/// Does not suport comments in the file.
/// if a file has more data after all the pixels, this function wouldn't see it (no error).
pub fn read_ppm(path_name: String) -> Result<Pimage, Error> {
    let a = fs::read(path_name)?;
    let mut iter = a.iter();

    let size = read_header(&mut iter)?;
    let mut image = Pimage::new(size.0, size.1, Color::WHITE);

    for y in 0..image.height() {
        for x in 0..image.width() {
            let new_color = read_next_color(&mut iter)?;
            unsafe {
                image.set_unchecked(x, y, new_color);
            }
        }
    }

    Ok(image)
}

/// Writes a Pimage to a PNM P6 (PPM) file.
///
/// A PNM P6 is an uncompressed raster image (as opposed to vector image).
pub fn write_ppm(path_name: String, pimage: &Pimage) -> Result<(), Error> {
    let mut contents = format!("P6\n{} {}\n255\n", pimage.width(), pimage.height()).into_bytes();
    let pixels: Vec<u8> = pimage
        .pixels()
        .clone()
        .iter()
        .flat_map(|c| vec![c.r, c.g, c.b])
        .collect();
    contents.extend(pixels);
    fs::write(path_name, contents)
}

/// Tries to read the header of a PNM P6 file, and returns the resolution of the image.
fn read_header(file: &mut std::slice::Iter<'_, u8>) -> Result<(usize, usize), Error> {
    if let Some(p) = file.next() {
        if *p != b'P' {
            return error("Wrong magical values on top of header, doesn't start with \"P\".");
        }
    } else {
        return error("Empty File");
    }
    if let Some(six) = file.next() {
        if *six != b'6' {
            return error("Wrong PNM file type, expects a PNM P6 (binary PPM) file.");
        }
    } else {
        return error("Almost Empty File");
    }
    if let Some(newline) = file.next() {
        if *newline != b'\n' {
            return error("Wrong file type, no newline after magic number P6.");
        }
    } else {
        return error("Incomplete file, nothing after magic number P6.");
    }
    let width = read_number(file)?;
    let height = read_number(file)?;
    if read_number(file)? != 255 {
        return error("Expects a maximum rgb value of 255.");
    }
    Ok((width, height))
}

/// Tries to read a usize number from an iterator over characters as u8.
fn read_number(file: &mut std::slice::Iter<'_, u8>) -> Result<usize, Error> {
    let mut number = 0;
    loop {
        let next_char = if let Some(next) = file.next() {
            next
        } else {
            return error("Reached end of file before having a full header.");
        };
        let next_digit = if let Some(digit) = to_digit(*next_char) {
            digit
        } else if (*next_char as char).is_whitespace() {
            return Ok(number);
        } else {
            return error("Wrong character in header.");
        };
        number = 10 * number + next_digit;
    }
}

/// Takes a character as a u8, and tries to return the digit as a number.
///
/// ex: '1' -> Some(1)
///     '4' -> Some(4)
///     'a' -> None
fn to_digit(char: u8) -> Option<usize> {
    if char < b'9' && char > b'0' {
        Some((char - b'0') as usize)
    } else {
        None
    }
}

/// Returns the next color in the file.
/// Assumes the header has been skipped.
fn read_next_color(file: &mut std::slice::Iter<'_, u8>) -> Result<Color, Error> {
    let mut color = Color::new(0, 0, 0);
    if let Some(red) = file.next() {
        color.r = *red;
    } else {
        return error("Reached end of file before reading the next red value.");
    }
    if let Some(green) = file.next() {
        color.g = *green;
    } else {
        return error("Reached end of file before reading the next green value.");
    }
    if let Some(blue) = file.next() {
        color.b = *blue;
    } else {
        return error("Reached end of file before reading the next blue value.");
    }
    Ok(color)
}
