use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_clique(
    connections: &HashMap<String, HashSet<String>>,
    potential_clique: &HashSet<String>,
) -> bool {
    for computer1 in potential_clique {
        if let Some(neighbors) = connections.get(computer1) {
            // Vérifier que computer1 est connecté à tous les autres ordinateurs du set
            for computer2 in potential_clique {
                if computer1 != computer2 && !neighbors.contains(computer2) {
                    return false;
                }
            }
        } else {
            return false;
        }
    }
    true
}

fn find_max_clique(connections: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    let mut max_clique: HashSet<String> = HashSet::new();
    let all_computers: HashSet<String> = connections.keys().cloned().collect();

    // On commence par des cliques de taille 1 et on augmente progressivement
    let mut current_clique: HashSet<String> = HashSet::new();

    fn extend_clique(
        current_clique: &mut HashSet<String>,
        remaining_computers: &HashSet<String>,
        max_clique: &mut HashSet<String>,
        connections: &HashMap<String, HashSet<String>>,
    ) {
        // Si la clique courante est plus grande que la maximale trouvée jusqu'ici
        if current_clique.len() > max_clique.len() {
            max_clique.clear();
            max_clique.extend(current_clique.iter().cloned());
        }

        // Pour chaque ordinateur restant
        for computer in remaining_computers.clone() {
            // Ajouter l'ordinateur à la clique courante
            current_clique.insert(computer.clone());

            // Vérifier si c'est toujours une clique valide
            if is_clique(connections, current_clique) {
                // Créer un nouveau set des ordinateurs restants qui sont connectés à tous
                let mut new_remaining: HashSet<String> = remaining_computers
                    .iter()
                    .filter(|&c| {
                        c != &computer
                            && connections.get(c).map_or(false, |neighbors| {
                                current_clique.iter().all(|pc| neighbors.contains(pc))
                            })
                    })
                    .cloned()
                    .collect();

                // Continuer récursivement
                extend_clique(current_clique, &new_remaining, max_clique, connections);
            }

            // Retirer l'ordinateur pour essayer d'autres combinaisons
            current_clique.remove(&computer);
        }
    }

    extend_clique(
        &mut current_clique,
        &all_computers,
        &mut max_clique,
        connections,
    );
    max_clique
}

fn main() -> io::Result<()> {
    let path = "23_input.txt";
    let mut data: Vec<(String, String)> = Vec::new();

    // Lecture du fichier
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                let mut parts = content.split("-");
                let computer1 = parts.next().unwrap().to_string();
                let computer2 = parts.next().unwrap().to_string();
                data.push((computer1, computer2));
            }
        }
    }

    // Construction du graphe
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for (computer1, computer2) in data {
        let set1 = connections
            .entry(computer1.clone())
            .or_insert(HashSet::new());
        set1.insert(computer2.clone());

        let set2 = connections
            .entry(computer2.clone())
            .or_insert(HashSet::new());
        set2.insert(computer1.clone());
    }

    // Trouver la plus grande clique
    let max_clique = find_max_clique(&connections);

    println!("Plus grand ensemble d'ordinateurs tous reliés entre eux:");
    println!("Taille: {}", max_clique.len());
    println!("Ordinateurs: {:?}", max_clique);

    let mut sorted_computers: Vec<String> = max_clique.into_iter().collect();
    sorted_computers.sort();
    let res = sorted_computers.join(",");

    println!("{res}");

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
