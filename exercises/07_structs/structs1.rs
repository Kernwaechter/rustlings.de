struct ColorRegularStruct {
    // TODO: Füge die Felder hinzu, die der Test `regular_structs` erwartet.
    // Welche Typen sollten die Felder haben? Was sind die minimalen und maximalen Werte für RGB-Farben?
}

struct ColorTupleStruct(/* TODO: Füge die Felder hinzu, die der Test `tuple_structs` erwartet */);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // Hier kannst du optional experimentieren.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Erzeuge ein reguläres Struct mit Werten (eine Instanz).
        // let green =

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Erzeuge ein Tupel-Struct mit Werten (eine Instanz).
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Erzeuge ein Unit-Struct (eine Instanz).
        // let unit_struct =
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
