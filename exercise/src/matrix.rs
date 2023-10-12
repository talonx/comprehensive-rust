fn main() {
    let matrix = [[10, 20, 30], [40, 50, 60], [70, 80, 90]];
    pretty_print(matrix);
    pretty_print(transpose(matrix));
}

fn test() {
   
    let array = [10, 20, 30];
    print!("Iterating over array");
    for n in &array {
        print!(" {n}");
    }
    println!();

    print!("Iterating over range");
    for i in 0..3 {
        print!(" {}", array[i]);
    }
    println!();
}

fn pretty_print(matrix: [[i32; 3]; 3]) {
    for n in 0..3 {
        for m in 0..3 {
            print!(" {}", matrix[n][m]);
        }
        println!();
    }
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut t = [[0; 3]; 3];
    for n in 0..3 {
        for m in 0..3 {
            t[m][n] = matrix[n][m];
        }
    }
    t
}
