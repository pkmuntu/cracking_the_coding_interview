fn is_protein(sequence: String) -> bool {
    if sequence.len() <= 1 {
        return true;
    } else {
        if sequence.chars().nth(0).unwrap() == sequence.chars().nth(sequence.len() - 1).unwrap() {
            return is_protein(sequence[1..sequence.len() - 1].to_string());
        }
    }
    return false;
}
fn main() {
    // Driver code

    let protein = String::from("acbca");
    println!(
        "{}{}{}{:}",
        "Is ",
        protein.clone(),
        " a Protein? = ",
        is_protein(protein)
    );
}

// Time complexity = O(n)
// Space complexity = O(n)
