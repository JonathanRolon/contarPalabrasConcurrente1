pub mod file_handler {

    use std::{
        collections::HashMap,
        fs::File,
        io::BufRead,
        io::BufReader,
        path::Path,
        sync::{Arc, Mutex},
        thread,
    };

    pub fn leer_archivo(filename: impl AsRef<Path>, ocurrencias_global: &mut HashMap<String, u16>) {
        
        let file = File::open(filename).expect("no such file");
        let reader = BufReader::new(file);
        let builder = thread::Builder::new();

        let join_handle: thread::JoinHandle<_> = builder
            .spawn(|| {
                
                let mut ocurrencias: HashMap<String, u16> = HashMap::new();
                for line in reader.lines() {
                    let line2 = line.unwrap();
                    let palabras: Vec<&str> = line2.split(" ").collect();
                    for palabra in palabras {
                        let pa2 = palabra.clone().to_lowercase().to_string();
                        match ocurrencias.get(&pa2) {
                            Some(cantidad) => {
                                ocurrencias.insert(pa2.clone(), *cantidad + 1);
                            }
                            None => {
                                ocurrencias.insert(pa2.clone(), 1);
                            }
                        }
                    }
                }

                ocurrencias.clone()
            })
            .unwrap();

        let joined = join_handle.join();
                
        for (key, value) in joined.unwrap() {
            
            match ocurrencias_global.get(&key) {
                Some(cantidad) => {
                    ocurrencias_global.insert(key.clone(), value + cantidad);
                }
                None => {
                    ocurrencias_global.insert(key.clone(), value);
                }
            }
        }

    }

    pub fn contar_palabras(filenames: Vec<String>) {

        let mut ocurrencias_global: HashMap<String, u16> = HashMap::new();

        for file in filenames {
            leer_archivo(file, &mut ocurrencias_global);
        }

        println!("{:?}", ocurrencias_global);
    }
}
