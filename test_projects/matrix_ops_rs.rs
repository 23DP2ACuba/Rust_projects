// fn getMatrixDet(matrix: Vec<Vec<i32>>) -> i32 {
//     let mut det = 0;
//     for row in &matrix.iter(){

//     }
//     det

// }

fn print_matrix(matrix: &Vec<Vec<i32>>){
    for row in matrix{
        print!("|");
        for elem in row{
            print!(" {} ", elem);
        }
        println!("|");
    }
}
fn main() {
    let mat = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    print_matrix(&mat);
}