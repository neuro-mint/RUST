
fn main() {
    is_palindrome("TOT".to_string());
    unique_characters("The world is a fishbowl".to_string());
    string_permutations("ATGC".to_string());
    char_freq("The sun shines equally on saints and beggers.".to_string(), 's');
    reverse_words("The quick brown fox  jumped over the lazy dog.".to_string());
    is_anagram("anagram", "aramang");
}

//check whether the given string is a palindrome or not
fn is_palindrome(s: String) {
    let reverse: String = s.chars().rev().collect();
    if s == reverse {
        println!("{} is a palindrome", s);
    } else {
        println!("{} is not a palindrome", s);
    }
}

//print the number of unique characters from a given string
fn unique_characters(s: String) {
    let chars: Vec<char> = s.chars().collect();
    let mut unique_chars: Vec<char> = Vec::new();
    for i in 0..chars.len() {
        match unique_chars.contains(&chars[i]) {
            true => continue, 
            false => unique_chars.push(chars[i]),
        } 
    }
    println!("{:?} unique characters present in > {:?}", unique_chars.len(), s);
}

//print all posible permutations of a given string
fn string_permutations(s: String) {
    let letters: Vec<char> = s.chars().collect(); 
    let len = letters.len();

    fn backtrack(mut l: Vec<char>, start: usize, n: usize) {
        //backtracking function to calculate permutations
        if start == n {
            let s: String = l.into_iter().collect();
            println!("{:?}", s);
            //append result to a vector
        } else {
            for i in start..=n {
                l.swap(n, i);
                backtrack(l.to_owned(), start + 1, n);
                l.swap(n, i);
            }
        }
    }

    backtrack(letters, 0, len-1);
}

//return the frequency of occurance of a given char from a given string
fn char_freq(s: String, x: char) {
    let mut freq = 0;
    for i in  s.chars(){
        if i == x {
            freq += 1;
        } else {
            continue;
        }
    }
    println!(" THe char {:?} appears {:?} times in the given string. ", x, freq)
}

//reverse the words in a string and print the string with the words in reerse order
fn reverse_words(s: String) {
    let mut s: Vec<String> = s
        .split_whitespace()
        .map(|s: &str| s.to_string())
        .collect();
    s.reverse();

    let mut s_final: String = String::new(); 
    for i in 0..s.len() {
        if i < s.len()-1 {
            s_final += &s[i].to_string();
            s_final += &" ".to_string();
        } else {
            s_final += &s[i].to_string();
        }
    }
    println!("{:?}", s_final);
}

//check if two strings are anagrams or not
fn is_anagram(s1: &str, s2: &str) {
    let mut s1: Vec<char> = s1.to_lowercase().chars().collect();
    s1.sort();
    let s1: String = s1.into_iter().collect();

    let mut s2: Vec<char> = s2.to_lowercase().chars().collect();
    s2.sort();
    let s2: String = s2.into_iter().collect();

    if s1 == s2 {
        println!("The strings {:?} & {:?} are anagrams.", s1, s2);
    } else {
        println!("The strings {:?} & {:?} are not anagrams.", s1, s2);
    }
}
