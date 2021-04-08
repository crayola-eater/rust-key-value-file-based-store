use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key must be provided!");
    let value = arguments.next().expect("Value must be provided!");
    println!("The key is '{}' and the value is '{}'", key, value);

    let mut database = Database::new().expect("Failed to create database");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);

    database.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
    flush: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();

        // read the kv.db file
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            // parse the string
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            // populate our map
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map, flush: false })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.flush = true;
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        std::fs::write("kv.db", contents)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if self.flush {
            return;
        }
        match self.flush() {
            Ok(()) => {}
            Err(e) => eprintln!("{}", e),
        }
    }
}
