use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

type Connections = HashMap<String, Vec<String>>;

fn canoncialize(first: &str, second: &str, third: &str) -> Vec<String> {
    let mut v = vec![first.to_owned(), second.to_owned(), third.to_owned()];
    v.sort();
    v
}

fn interconnections(connections: &Connections, first: &str) -> Vec<Vec<String>> {
    connections[first]
        .iter()
        .flat_map(|second| {
            connections[first]
                .iter()
                .map(move |third| (first, second, third))
        })
        .filter(|&(_, second, third)| connections[second].contains(third))
        .map(|(first, second, third)| canoncialize(first, second, third))
        .collect()
}

fn count_interconnected_computers_with_t(connections: &Connections) -> usize {
    let connected: HashSet<Vec<String>> = HashSet::from_iter(
        connections
            .keys()
            .filter(|first| first.starts_with("t"))
            .flat_map(|first| interconnections(connections, first)),
    );

    connected.len()
}

fn bron_kerbosch(
    connections: &Connections,
    r: &HashSet<String>,
    p: &HashSet<String>,
    x: &HashSet<String>,
    maximal_cliques: &mut HashSet<Vec<String>>,
) {
    if p.is_empty() && x.is_empty() {
        let mut clique = r.iter().cloned().collect::<Vec<_>>();
        clique.sort();
        maximal_cliques.insert(clique);
    }

    let mut p = p.clone();
    let mut x = x.clone();
    let mut r = r.clone();

    for v in p.clone().iter() {
        let neighbors = HashSet::from_iter(connections[v].iter().cloned());

        let pcopy = HashSet::from_iter(p.intersection(&neighbors).cloned());
        let xcopy = HashSet::from_iter(x.intersection(&neighbors).cloned());
        r.insert(v.clone());
        bron_kerbosch(connections, &r, &pcopy, &xcopy, maximal_cliques);
        r.remove(v);
        p.remove(v);
        x.insert(v.clone());
    }
}

fn largest_interconnected_set(connections: &Connections) -> String {
    let mut maximal_cliques = HashSet::new();
    bron_kerbosch(
        connections,
        &HashSet::new(),
        &HashSet::from_iter(connections.keys().cloned()),
        &HashSet::new(),
        &mut maximal_cliques,
    );
    let max_set_size = maximal_cliques.iter().map(|set| set.len()).max().unwrap();
    let max_set = maximal_cliques
        .iter()
        .find(|&set| set.len() == max_set_size)
        .unwrap();

    let mut max_vec: Vec<String> = Vec::from_iter(max_set.iter().cloned());
    max_vec.sort();
    max_vec.join(",")
}

fn parse_input(data: &str) -> Connections {
    let lines = data.lines();
    let pairs = lines.map(|line| line.split_once('-').unwrap());

    let mut map = Connections::new();

    for (a, b) in pairs {
        map.entry(a.to_owned()).or_default().push(b.to_owned());
        map.entry(b.to_owned()).or_default().push(a.to_owned());
    }

    map
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let input_file = if args.len() >= 2 {
        std::path::PathBuf::from(&args[1])
    } else {
        let exe = std::env::current_exe().unwrap();
        exe.parent()
            .unwrap()
            .join("../..")
            .join(exe.file_name().unwrap())
            .join("src/main.txt")
    };
    let data = read_to_string(&input_file).unwrap();
    let connections = parse_input(&data);

    println!(
        "count of interconnected computers with t: {}",
        count_interconnected_computers_with_t(&connections)
    );
    println!(
        "largest interconnected set: {}",
        largest_interconnected_set(&connections)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let connections: HashMap<String, Vec<String>> = parse_input(&data);
        assert_eq!(count_interconnected_computers_with_t(&connections), 7);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let connections: HashMap<String, Vec<String>> = parse_input(&data);
        assert_eq!(count_interconnected_computers_with_t(&connections), 1215);
    }

    #[test]
    fn test_part2() {
        let data = read_to_string("src/test.txt").unwrap();
        let connections: HashMap<String, Vec<String>> = parse_input(&data);
        assert_eq!(largest_interconnected_set(&connections), "co,de,ka,ta");
    }

    #[test]
    fn answer_part2() {
        let data = read_to_string("src/main.txt").unwrap();
        let connections: HashMap<String, Vec<String>> = parse_input(&data);
        assert_eq!(
            largest_interconnected_set(&connections),
            "bm,by,dv,ep,ia,ja,jb,ks,lv,ol,oy,uz,yt"
        );
    }
}
