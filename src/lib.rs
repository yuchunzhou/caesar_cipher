#[cfg(test)]
mod tests {
    use crate::{decrypt, encrypt};

    #[test]
    fn it_works() {
        let message = "helloworld".to_owned();
        assert_eq!(decrypt(encrypt(message.clone(), 2), 2), message);
    }
}

pub fn encrypt(content: String, offset: i32) -> String {
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
        }
    }

    result
}

pub fn decrypt(content: String, offset: i32) -> String {
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
        }
    }

    result
}
