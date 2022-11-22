fn main() {
    let vector_a = [1, 2, 3];
    let vector_b: [u8; 3] = [1, 2, 3];
    let blank_vector1 = [0; 3];
    let blank_vector2: [u8; 3] = [0; 3];

    let arrays = [vector_a, vector_b, blank_vector1, blank_vector2];

    for element in &arrays {
        print!("{:?}: ", element);

        for n in element.iter() {
            print!("\t{} + 3 = {}", n, n + 3);
        }

        let mut sum = 0;
        for i in 0..element.len() {
            sum += element[i];
        }
        println!("\t(Sigma{:?} = {}", element, sum);
    }
}
