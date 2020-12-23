fn main() -> Result<(), String> {
    let cups = &[5, 8, 3, 9, 7, 6, 2, 4, 1];

    let cups_after_100_steps = result_puzzle_1(&do_n_steps(100, cups)?)?;
    print!("After 100 steps: ");
    for label in cups_after_100_steps {
        print!("{}", label);
    }
    println!();

    // There are probably better ways. But I will try the brute force way first.
    let million_cups: Vec<u64> = cups.iter().copied().chain(10..=1_000_000).collect();
    let cups_after_ten_million_steps = do_n_steps(10_000_000, &million_cups)?;
    let star_cup_product = result_puzzle_2(&cups_after_ten_million_steps)?;
    println!("The product of the star cups is: {}", star_cup_product);

    Ok(())
}

fn do_n_steps(n: usize, cups: &[u64]) -> Result<Vec<(usize, u64)>, String> {
    let mut links: Vec<(usize, u64)> = cups
        .iter()
        .enumerate()
        .map(|(i, label)| ((i + 1) % cups.len(), *label))
        .collect();
    let mut current = 0;
    for _ in 0..n {
        current = do_step(&mut links as &mut [(usize, u64)], current)?;
    }

    Ok(links)
}

fn result_puzzle_1(links: &[(usize, u64)]) -> Result<Vec<u64>, String> {
    let mut index = links
        .iter()
        .find(|(_, label)| *label == 1)
        .ok_or_else(|| "Unable to find cup with label 1".to_owned())?
        .0;

    let mut result: Vec<u64> = Vec::with_capacity(links.len() - 1);
    while links[index].1 != 1 {
        result.push(links[index].1);
        index = links[index].0;
    }
    Ok(result)
}

fn result_puzzle_2(links: &[(usize, u64)]) -> Result<u64, String> {
    let index1 = links
        .iter()
        .find(|(_, label)| *label == 1)
        .ok_or_else(|| "Unable to find cup with label 1".to_owned())?
        .0;
    let (index2, label1) = links[index1];
    let label2 = links[index2].1;
    Ok(label1 * label2)
}

fn do_step(links: &mut [(usize, u64)], current: usize) -> Result<usize, String> {
    let first_removed_index = links[current].0;
    let first_removed: (usize, u64) = links[first_removed_index];
    let second_removed: (usize, u64) = links[first_removed.0];
    let third_removed: (usize, u64) = links[second_removed.0];
    let removed_labels = [first_removed.1, second_removed.1, third_removed.1];
    links[current].0 = third_removed.0;

    let destination_index = find_destination_index(links, current, &removed_labels)?;
    let prev_destination_successor = links[destination_index].0;
    links[destination_index].0 = first_removed_index;
    links[second_removed.0].0 = prev_destination_successor;

    Ok(links[current].0)
}

fn find_destination_index(
    links: &[(usize, u64)],
    current: usize,
    removed_labels: &[u64],
) -> Result<usize, String> {
    let mut destination_label = next_destination_label(links[current].1, links.len() as u64);
    while removed_labels.contains(&destination_label) {
        destination_label = next_destination_label(destination_label, links.len() as u64);
    }
    find_label_index(links, destination_label)
        .ok_or_else(|| format!("Unable to find destination label {}", destination_label))
}

fn find_label_index(links: &[(usize, u64)], label: u64) -> Option<usize> {
    if label < 10 {
        links
            .iter()
            .enumerate()
            .find(|(_, (_, l))| *l == label)
            .map(|(i, _)| i)
    } else {
        Some(label as usize - 1)
    }
}

fn next_destination_label(label: u64, max_label: u64) -> u64 {
    if label <= 1 {
        max_label
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
