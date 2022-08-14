#[cfg(test)]
mod tests {
    use crate::{decrypt, encrypt};

    #[test]
    fn it_works() {
        let message1 = "helloworld";
        assert_eq!(
            &decrypt(&encrypt(message1, 2).unwrap(), 2).unwrap(),
            message1
        );

        let message2 = "你好世界";
        assert!(encrypt(message2, 2).is_err());

        let message3 = "❤️";
        assert!(decrypt(message3, 2).is_err());
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CipherError {
    #[error("non english character(s) found")]
    CheckError,
}

type Result<T> = std::result::Result<T, CipherError>;

pub fn encrypt(content: &str, offset: i32) -> Result<String> {
    let mut result = String::new();

    for c in content.chars() {
        if c.is_lowercase() {
            let i = c as i32 + offset;
            if i > 122 {
                let i = 96 + i - 122;
                result.push(i as u8 as char);
            } else if i < 97 {
                let i = 96 + 97 - i;
                result.push(i as u8 as char);
            } else {
                result.push(i as u8 as char);
            }
        } else if c.is_uppercase() {
            let i = c as i32 + offset;
            if i > 90 {
                let i = 64 + i - 90;
                result.push(i as u8 as char);
            } else if i < 65 {
                let i = 64 + 65 - i;
                result.push(i as u8 as char);
            } else {
                result.push(i as u8 as char);
            }
        } else {
            return Err(CipherError::CheckError);
        }
    }

    Ok(result)
}

pub fn decrypt(content: &str, offset: i32) -> Result<String> {
    let mut result = String::new();

    for c in content.chars() {
        if c.is_lowercase() {
            let i = c as i32 - offset;
            if i < 97 {
                let i = 122 - (offset - (c as i32 - 97)) + 1;
                result.push(i as u8 as char);
            } else {
                result.push(i as u8 as char);
            }
        } else if c.is_uppercase() {
            let i = c as i32 - offset;
            if i < 65 {
                let i = 90 - (offset - (c as i32 - 65)) + 1;
                result.push(i as u8 as char);
            } else {
                result.push(i as u8 as char);
            }
        } else {
            return Err(CipherError::CheckError);
        }
    }

    Ok(result)
}
