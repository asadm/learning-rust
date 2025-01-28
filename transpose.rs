type Mat3 = [[i32; 3]; 3];

fn transpose(matrix: Mat3) -> Mat3{
    let mut result: Mat3 = [[0;3];3];
    for i in 0..3{
        for j in 0..3{
            result[j][i] = matrix[i][j];
        }
    }
    result
}

fn main(){
    let matrix = [[1,2,3],[4,5,6],[7,8,9]];
    dbg!(matrix);
    let transposed = transpose(matrix);
    dbg!(transposed);
    
    assert!(transposed[0] == [1, 4, 7]);
    assert!(transposed[1] == [2, 5, 8]);
    assert!(transposed[2] == [3, 6, 9]);
}