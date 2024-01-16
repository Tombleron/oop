use std::{fmt::Display, io::Write};


#[derive(Debug)]
struct Album {
    title: String,
    artist: String,
    year: u32,
}

impl Album {
    fn new(title: String, artist: String, year: u32) -> Self {
        Self {
            title,
            artist,
            year,
        }
    }
}

impl Display for Album {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} by {} ({})", self.title, self.artist, self.year)
    }
}


fn main() {

    let mut albums = [
        Album::new("The Silver Cord".to_string(), "King Gizzard and The Lizard Wizard".to_string(), 2023),
        Album::new("PetroDragonic Apocalypse; or, Dawn of eternal Night: An Annihilation of Planet Earth and the Beginning of Merciless Damnation".to_string(), "King Gizzard and The Lizard Wizard".to_string(), 2023),
        Album::new("Changes".to_string(), "King Gizzard and The Lizard Wizard".to_string(), 2022),
        Album::new("Laminated Denim".to_string(), "King Gizzard and The Lizard Wizard".to_string(), 2022),
        Album::new("Ice, Death, Planets, Lungs, Mushrooms and Lava".to_string(), "King Gizzard and The Lizard Wizard".to_string(), 2022),
        Album::new("Made in Timeland".to_string(), "King Gizzard and The Lizard Wizard".to_string(), 2022),
        Album::new("Omnium Gatherum".to_string(), "King Gizzard and The Lizard Wizard".to_string(), 2022),
        Album::new("Butterfly 3000".to_string(), "King Gizzard and The Lizard Wizard".to_string(), 2021),
        Album::new("L.W.".to_string(), "King Gizzard and The Lizard Wizard".to_string(), 2021),
        Album::new("Infest the Rats' Nest".to_string(), "King Gizzard and The Lizard Wizard".to_string(), 2019),
        Album::new("Fishing for Fishies".to_string(), "King Gizzard and The Lizard Wizard".to_string(), 2019),
        Album::new("K.G.".to_string(), "King Gizzard and The Lizard Wizard".to_string(), 2019),
        Album::new("Polygondwanaland".to_string(), "King Gizzard and The Lizard Wizard".to_string(), 2017),
        Album::new("Gumboot Soup".to_string(), "King Gizzard and The Lizard Wizard".to_string(), 2017),
        Album::new("Sketches of Brunswick East".to_string(), "King Gizzard and The Lizard Wizard".to_string(), 2017),
    ];

    albums.sort_by(|a, b| a.year.cmp(&b.year));
    write_to_file("year.txt", &albums).unwrap();
    
    albums.sort_by(|a, b| a.title.cmp(&b.title));
    write_to_file("title.txt", &albums).unwrap();
}

fn write_to_file(path: &str, albums: &[Album]) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    for album in albums {
        writeln!(file, "{}", album)?;
    }
    Ok(())
}
