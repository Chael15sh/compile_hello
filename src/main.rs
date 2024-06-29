use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    // Créer le fichier C "hello.c"
    let mut file = File::create("hello.c").expect("Impossible de créer le fichier hello.c");
    let c_code = r#"
    #include <stdio.h>

    int main() {
        printf("Hello, World!\n");
        return 0;
    }
    "#;
    file.write_all(c_code.as_bytes()).expect("Impossible d'écrire dans le fichier");

    // Compiler le fichier C en exécutable
    let output = Command::new("gcc")
        .arg("hello.c")
        .arg("-o")
        .arg("hello")
        .output()
        .expect("Échec de la compilation");

    if output.status.success() {
        println!("Compilation réussie !");
    } else {
        println!("Erreur de compilation :");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}
