// TODO: Behebe den Compiler-Fehler über den Aufruf einer privaten Funktion.
mod sausage_factory {
    // Lass niemanden außerhalb dieses Moduls das hier sehen!
    fn get_secret_recipe() -> String {
        String::from("Ingwer")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("Wurst!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
