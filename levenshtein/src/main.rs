use std::{fs::File, io::Read, thread};


fn read_binary_file(file_path:&str)-> std::io::Result<Vec<u8>>{
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn levenshtein(a: &[u8], b: &[u8]) -> usize {
    let len_a = a.len();
    let len_b = b.len();
    let mut matrix = vec![vec![0; len_b + 1]; len_a + 1];

    for i in 0..=len_a {
        matrix[i][0] = i;
    }
    for j in 0..=len_b {
        matrix[0][j] = j;
    }

    for i in 1..=len_a {
        for j in 1..=len_b {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            matrix[i][j] = *[
                matrix[i - 1][j] + 1,          // deletion
                matrix[i][j - 1] + 1,          // insertion
                matrix[i - 1][j - 1] + cost,   // substitution
            ].iter().min().unwrap();
        }
    }

    matrix[len_a][len_b]
}
fn optimized_levenshtein(a: &[u8], b: &[u8]) -> usize {
    let len_a = a.len();
    let len_b = b.len();

    if len_a == 0 {
        return len_b;
    }
    if len_b == 0 {
        return len_a ;
    }

    let mut prev_row = (0..=len_b).collect::<Vec<usize>>();
    let mut curr_row = vec![0; (len_b + 1).try_into().unwrap()];

    for i in 1..=len_a {
        curr_row[0] = i;
        for j in 1..=len_b {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            curr_row[j] = *[
                prev_row[j] + 1,          // deletion
                curr_row[j - 1] + 1,      // insertion
                prev_row[j - 1] + cost,   // substitution
            ].iter().min().unwrap();
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    prev_row[len_b]
}

fn chunked_levenshtein(a: &[u8], b: &[u8], chunk_size: usize) -> usize {
    let mut total_distance = 0;
    let mut i = 0;

    while i < a.len() && i < b.len() {
        let end = usize::min(i + chunk_size, a.len());
        let a_chunk = &a[i..end];
        let b_chunk = &b[i..end];
        total_distance += optimized_levenshtein(a_chunk, b_chunk);
        i += chunk_size;
        println!("i is: {}", i);
    }

    if i < a.len() {
        total_distance += a.len() - i;
    }
    if i < b.len() {
        total_distance += b.len() - i;
    }

    total_distance
}

fn parallel_levenshtein(a: Vec<u8>, b: Vec<u8>, num_threads: usize) -> usize {
    let chunk_size = (a.len() / num_threads).max(1);
    let mut handles = vec![];

    for chunk_start in (0..a.len()).step_by(chunk_size) {
        let a_chunk = a[chunk_start..usize::min(chunk_start + chunk_size, a.len())].to_vec();
        let b_chunk = b.clone();
        let handle = thread::spawn(move || optimized_levenshtein(&a_chunk, &b_chunk));
        handles.push(handle);
    }

    let mut total_distance = 0;
    for handle in handles {
        total_distance += handle.join().unwrap();
    }

    total_distance
}


fn main() -> std::io::Result<()> {
    // let file1 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/Rust/LinkedList/linked_link_opt0/target/debug/linked_link_opt0";
    // let file2 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/Rust/LinkedList/linked_list_opt1/target/debug/linked_list_opt1";

    let file1 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/Files/file1.txt";
    let file2 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/Files/file2.txt";


    let data1 = read_binary_file(file1)?;
    let data2 = read_binary_file(file2)?;

    let distance = levenshtein(&data1, &data2);
    // let distance = optimized_levenshtein(&data1, &data2);
    // let distance = parallel_levenshtein(data1, data2, 2); // Using 4 threads

    // let distance = chunked_levenshtein(&data1, &data2, 1024); // Using 4 threads

    println!("The Levenshtein distance between the two files is: {}", distance);


    Ok(())
}

