fn main() -> Result<(), String> {
    let cups = &[5, 8, 3, 9, 7, 6, 2, 4, 1];

    let cups_after_100_steps = do_n_steps(100, cups)?;
    print!("After 100 steps: ");
    for label in cups_after_100_steps {
        print!("{}", label);
    }
    println!();

    Ok(())
}

fn do_n_steps(n: usize, cups: &[u8]) -> Result<Vec<u8>, String> {
    let mut links: Vec<(usize, u8)> = cups
        .iter()
        .enumerate()
        .map(|(i, label)| ((i + 1) % cups.len(), *label))
        .collect();
    let mut current = 0;
    for _ in 0..n {
        current = do_step(&mut links as &mut [(usize, u8)], current)?;
    }
    let mut index = links
        .iter()
        .find(|(_, label)| *label == 1)
        .ok_or_else(|| "Unable to find cup with label 1".to_owned())?
        .0;

    let mut result: Vec<u8> = Vec::with_capacity(cups.len() - 1);
    while links[index].1 != 1 {
        result.push(links[index].1);
        index = links[index].0;
    }
    Ok(result)
}

fn do_step(links: &mut [(usize, u8)], current: usize) -> Result<usize, String> {
    let first_removed_index = links[current].0;
    let first_removed: (usize, u8) = links[first_removed_index];
    let second_removed: (usize, u8) = links[first_removed.0];
    let third_removed: (usize, u8) = links[second_removed.0];
    let removed_labels = [first_removed.1, second_removed.1, third_removed.1];
    links[current].0 = third_removed.0;

    let destination_index = find_destination_index(links, current, &removed_labels)?;
    let prev_destination_successor = links[destination_index].0;
    links[destination_index].0 = first_removed_index;
    links[second_removed.0].0 = prev_destination_successor;

    Ok(links[current].0)
}

fn find_destination_index(
    links: &[(usize, u8)],
    current: usize,
    removed_labels: &[u8],
) -> Result<usize, String> {
    let mut destination_label = next_destination_label(links[current].1);
    while removed_labels.contains(&destination_label) {
        destination_label = next_destination_label(destination_label);
    }
    links
        .iter()
        .enumerate()
        .find(|(_, (_, label))| *label == destination_label)
        .map(|(i, _)| i)
        .ok_or_else(|| format!("Unable to find destination label {}", destination_label))
}

fn next_destination_label(label: u8) -> u8 {
    if label <= 1 {
        9
    } else {
        label - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_n_steps_works_for_example_10() {
        // given
        let cups = &[3, 8, 9, 1, 2, 5, 4, 6, 7];

        // when
        let result = do_n_steps(10, cups).expect("Expected success");

        // then
        assert_eq!(&result, &[9, 2, 6, 5, 8, 3, 7, 4])
    }
}
