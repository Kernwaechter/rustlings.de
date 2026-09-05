mod sausage_factory {
    fn get_secret_recipe() -> String {
        String::from("Ingwer")
    }

    // `pub` vor `fn` hinzugefügt, um die Funktion außerhalb des Moduls zugänglich zu machen.
    pub fn make_sausage() {
        get_secret_recipe();
        println!("Wurst!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
