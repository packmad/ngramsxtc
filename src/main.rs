use std::env;
use std::path::Path;


fn jprint_ngrams(v: &Vec<u8>) {
    let windows = vec![4, 5, 6];
    let last_elem_index = v.len() - 1;
    print!("{{");
    for s in &windows {
        print!("\"{}\":[", s);
        let mut i = 0;
        while i < last_elem_index {
            let end = i + s;
            if end > last_elem_index {
                let mut ngram = v[i..last_elem_index].to_vec();
                let mut padding: i8 = (s - (last_elem_index - i)).try_into().unwrap();
                while padding > 0 {
                    ngram.push(0);
                    padding -= 1;
                }
                print!("{:?}", ngram);
                break;
            }
            let ngram = &v[i..end];
            if end == last_elem_index {
                print!("{:?}", ngram);
            }
            else {
                print!("{:?},", ngram);
            }
            i += s;
        }
        if s == windows.last().unwrap() {
            print!("]");
        }
        else {
            print!("],");
        }
    }
    print!("}}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file_path>", args[0]);
        return;
    }
    let file_path = &args[1];
    if !Path::new(file_path).exists() {
        println!("{} not exists!", file_path);
        return;
    }
    match std::fs::read(file_path) {
        Ok(bytes) => { jprint_ngrams(&bytes) },  
        Err(_e) => { eprintln!("Can't open: {}", file_path); }
    }
}
