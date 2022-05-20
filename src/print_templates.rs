// Note, this file doesn't have any real use, I just wanted to have at least one extra module in my code

pub fn print_title() {
    let lines = [
        "-----------------------------------------------------",
        "|  __  __    __    ____     __    ____  ____  ___   |",
        "| (  \\/  )  /__\\  (  _ \\   (  )  (_  _)(  _ \\/ __)  |",
        "|  )    (  /(__)\\  )(_) )   )(__  _)(_  ) _ <\\__ \\  |",
        "| (_/\\/\\_)(__)(__)(____/   (____)(____)(____/(___/  |",
        "|                                                   |",
        "-----------------------------------------------------"
    ];    

    for line in lines {
        println!("{}", line);
    }
}