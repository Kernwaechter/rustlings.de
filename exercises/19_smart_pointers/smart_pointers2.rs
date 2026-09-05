// In dieser Übung wollen wir das Konzept mehrerer Besitzer über den Typ
// `Rc<T>` ausdrücken. Das hier ist ein Modell unseres Sonnensystems – es
// gibt einen Typ `Sun` und mehrere `Planet`s. Die Planeten übernehmen
// (Ownership) die Sonne, was ausdrückt, dass sie um die Sonne kreisen.

use std::rc::Rc;

#[derive(Debug)]
struct Sun;

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hallo von {self:?}!");
    }
}

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc1() {
        let sun = Rc::new(Sun);
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 1 Referenz

        let mercury = Planet::Mercury(Rc::clone(&sun));
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 2 Referenzen
        mercury.details();

        let venus = Planet::Venus(Rc::clone(&sun));
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 3 Referenzen
        venus.details();

        let earth = Planet::Earth(Rc::clone(&sun));
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 4 Referenzen
        earth.details();

        let mars = Planet::Mars(Rc::clone(&sun));
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 5 Referenzen
        mars.details();

        let jupiter = Planet::Jupiter(Rc::clone(&sun));
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 6 Referenzen
        jupiter.details();

        // TODO
        let saturn = Planet::Saturn(Rc::new(Sun));
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 7 Referenzen
        saturn.details();

        // TODO
        let uranus = Planet::Uranus(Rc::new(Sun));
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 8 Referenzen
        uranus.details();

        // TODO
        let neptune = Planet::Neptune(Rc::new(Sun));
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 9 Referenzen
        neptune.details();

        assert_eq!(Rc::strong_count(&sun), 9);

        drop(neptune);
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 8 Referenzen

        drop(uranus);
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 7 Referenzen

        drop(saturn);
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 6 Referenzen

        drop(jupiter);
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 5 Referenzen

        drop(mars);
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 4 Referenzen

        // TODO
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 3 Referenzen

        // TODO
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 2 Referenzen

        // TODO
        println!("Referenzzähler = {}", Rc::strong_count(&sun)); // 1 Referenz

        assert_eq!(Rc::strong_count(&sun), 1);
    }
}
