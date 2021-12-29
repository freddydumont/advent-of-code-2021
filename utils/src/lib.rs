/// converts Vec<&str> to Vec<String>
pub fn convert_vec_strings(input: Vec<&str>) -> Vec<String> {
    input.iter().map(|x| x.to_string()).collect::<Vec<String>>()
}
