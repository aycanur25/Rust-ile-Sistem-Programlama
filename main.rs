/*
    Bu terminal uygulaması ile bazı çevre değişkenlerin(environment variables)
    değerlerini ekrana yazdırmayı planlıyoruz.

*/
use std::{env, process};

fn main() {
      // Command line argümanlarına ulaşmak için args() metodu
    // Sistem çevre değişkenlerine (key,value) çiftleri ulaimak için vars() metodu

    // Komut satırından argüman alalım
    // --env yazılmışsa tüm çevre değişkenlerini alalım (--env args() üstünden yakalanabilir
    let args = env::args().collect::<Vec<String>>();

    // Programın mutlaka en az bir parametre ile çalışması gerektiğinde küçük bir kontrol ekledik.
    if args.len() < 2 {
        print_help();
        process::exit(1);
    }

    // Elde edilen argümanlardan 1nci indistekinin literal string değerinin
    // --env olup olmadığına bakıyoruz
    match args[1].as_str() {
        "-e" | "-env" => {
            for (key, value) in env::vars() {
                println!("{}:\t{}", key, value);
            }
        }
        "-h" | "-help" => {
            print_help();
        }
        "-w" | "-cwd" => {
            match env::current_dir() {
                Ok(dir) => println!("Current working directory: {}", dir.display()),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "-u" | "-usr" => {
            match env::var("USERNAME") {
                Ok(user_name) => println!("Username: {}", user_name),
                Err(_) => eprintln!("Could not retrieve username"),
            }
        }
        _ => {
            print_help();
        }
    }
}

/// Aracın nasıl kullanıldığını gösteren metot
fn print_help() {
    // Eğer argüman dizisinin eleman sayısı 2den azsa
    // Ekrana programın nasıl kullanıldığını yazdıralım
    println!("SysCo - A lightweight system tool\n");
    println!("Usage: sysco <argument>");
    println!("Valid Arguments:");
    println!("  -h, -help : Display usage");
    println!("  -e, -env  : Show env values");
    println!("  -w, -cwd  : Show the current working directory");
    println!("  -u, -usr  : Current root user");
    println!("For details www.azon.com/sysco.html");
}