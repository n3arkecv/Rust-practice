fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = chars.len().saturating_sub(1);

    while left < right {
        while left < right && !chars[left].is_ascii_alphanumeric() {
            left += 1;
        }
        while left < right && !chars[right].is_ascii_alphanumeric() {
            if right == 0 { break; } // 防止 underflow
            right -= 1;
        }
        if left < right {
            let cl = chars[left].to_ascii_lowercase();
            let cr = chars[right].to_ascii_lowercase();
            if cl != cr {
                return false;
            }
            left += 1;
            if right == 0 {
                break; 
                           }
            right -= 1;
        }
    }
    true
}

fn main() {
    let s1 = "A man, a plan, a canal: Panama";
    let s2 = "race a car";
    let s3 = "0P";

    println!("\"{}\" → {}", s1, is_palindrome(s1)); // true
    println!("\"{}\" → {}", s2, is_palindrome(s2)); // false
    println!("\"{}\" → {}", s3, is_palindrome(s3)); // false
}