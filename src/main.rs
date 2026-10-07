use std::println;

fn main(){
    let numbers = [12, 7, 19, 4, 7, 23, 12, 9, 4, 15];
    analyze_numbers(numbers);

    let matrix = [
                                    [4, 8, 2, 1],
                                    [7, 3, 9, 5],
                                    [6, 9, 8, 4],
                                    [1, 5, 3, 7],
                                ];
    matrix_analyzer(matrix);
}

fn analyze_numbers(numbers: [i32; 10]){
    let mut start = 0;
    let mut largest = 0;
    let mut smallest = numbers[0];

    let mut even = 0;
    let mut odd = 0;

    while start < numbers.len() {
         if numbers[start] > largest {
            largest = numbers[start];
        } 
        
        if numbers[start] < smallest{
            smallest = numbers[start];
        }

        if numbers[start] % 2 == 0 {
            even += 1;
        }

        if numbers[start] % 2 == 1 {
            odd += 1;
        }

        start += 1;
    }
    
    println!("The Largest Number is {}", largest);
    println!("The Smallest Number is {}", smallest);
    println!("We have {} Even Numbers", even);
    println!("We have {} Odd Numbers", odd);

     let mut current = 0;
        for num in numbers {
            current = current + num
        }
    
    let average = current / start as i32;
    println!("The Average of the numbers is {}", average);
   
}


fn matrix_analyzer(matrix: [[i32; 4]; 4]){
    let mut largest= matrix[0][0];
    let mut smallest = matrix[0][0];

    let mut twos = 0;

    for i in 0..matrix.len() {
        let mut row_sum = 0;
        let mut col_sum = 0;        

        for j in 0..matrix[i].len(){
            print!("{} ", matrix[i][j]); // Prints the Matrix Neatly
            row_sum = row_sum + matrix[i][j]; // Prints each Row
            col_sum = col_sum + matrix[j][i]; // Prints each Col

            if largest < matrix[i][j]{
                largest = matrix[i][j];
            }

            if smallest > matrix[i][j]{
                smallest = matrix[i][j];
            }

            if matrix[i][j] % 2 == 0{
                twos += 1;
            }

        }
        
        print!("The Diagonal of the Matrix is ");

        for i in 0..matrix.len() {
            print!("{} ", matrix[i][i]);
        }


        println!("");
        println!("This row's sum is {}", row_sum);
        println!("This column's sum is {}", col_sum);
        println!("");

        
    }

    println!("The Largest Number in the Matrix is {}", largest);
    println!("The Smallest Number in the Matrix is {}", smallest);
    println!("The Number of Even Numbers in the Matrix is {}", twos);
}
