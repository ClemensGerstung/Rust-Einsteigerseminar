/*
DE:
Rons Spell Helper
Schreibe eine App die es Ron erlaubt, in seinem Zauberspruchbuch:
 * Einen vorhandenen Eintrag anzuzeigen
 * Alle vorhandenen Zauberspruchnamen auszugeben
 * Einen neuen Zauberspruch einzutragen
 * Einen Zauberspruch zu entfernen
 * Nach dem das Programm die gewünschte Aufgabe erledigt hat, soll es erneut Fragen, welche Aktion ausgeführt werden
   soll, bis die Nutzer*in sich entscheidet das Programm zu verlassen

Erstelle hierzu:
- ein struct Spell mit den Feldern name, description
- ein struct Spellbook mit einem Vektor von Spells
- implementiere die Funktionen:
  - new: Ersellt eine neues Spellbook mit 5 Spells:
        Spell { name: "Expelliarmus".to_string(), description: "Entwaffnungszauber".to_string() },
        Spell { name: "Lumos".to_string(), description: "Beleuchtungszauber".to_string() },
        Spell { name: "Wingardium Leviosa".to_string(), description: "Schwebezauber".to_string() },
        Spell { name: "Accio".to_string(), description: "Aufrufezauber".to_string() },
        Spell { name: "Nox".to_string(), description: "Verdunklungszauber".to_string() },
  - im trait Write:
    - add_spell: Fügt einen neuen Spell zum Spellbook hinzu
    - remove_spell: Entfernt einen Spell aus dem Spellbook
  - im trait Read:
    - list_spells: Gibt alle Spellnamen aus dem Spellbook aus
    - find_spell: Gibt einen Spell aus dem Spellbook aus

EN:
Ron's Spell Helper
Write an app that allows Ron to display an existing entry in his spell book:
 * Display an existing entry
 * Display all existing spell names
 * Add a new spell
 * Remove a spell
 * After the program has completed the desired task, it should ask again which action is to be carried out until the user has completed it.
   until the user decides to exit the program

To do this, create
- a struct Spell with the fields name, description
- a struct Spellbook with a vector of spells
- implement the functions:
  - new: Creates a new spellbook with 5 spells:
        Spell { name: "Expelliarmus".to_string(), description: "Disarm Spell".to_string() },
        Spell { name: "Lumos".to_string(), description: "Illumination spell".to_string() },
        Spell { name: "Wingardium Leviosa".to_string(), description: "Levitation spell".to_string() },
        Spell { name: "Accio".to_string(), description: "Invocation spell".to_string() },
        Spell { name: "Nox".to_string(), description: "Darkening spell".to_string() },
    - in the trait Write:
        - add_spell: Adds a new spell to the spellbook
        - remove_spell: Removes a spell from the spellbook
    - in the trait Read:
        - list_spells: Returns all spell names from the spellbook
        - find_spell: Returns a spell from the spellbook

*/

struct Spell {
  name: String,
  description: String,
}

impl std::fmt::Display for Spell {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}: {}", self.name, self.description)
  }
}

struct Spellbook {
  spells: Vec<Spell>
}

trait Write {
  fn add_spell(&mut self, spell: Spell) -> bool;
  fn remove_spell(&mut self, spell_name: String) -> bool;
}

trait Read {
  fn list_spells(&self) -> Vec<String>;
  fn find_spell(&self, name: String) -> Option<&Spell>;
}

impl Spellbook {
  fn new() -> Self {
      Spellbook {
          spells: vec![
              Spell { name: "Expelliarmus".to_string(), description: "Disarm Spell".to_string() },
              Spell { name: "Lumos".to_string(), description: "Illumination spell".to_string() },
              Spell { name: "Wingardium Leviosa".to_string(), description: "Levitation spell".to_string() },
              Spell { name: "Accio".to_string(), description: "Invocation spell".to_string() },
              Spell { name: "Nox".to_string(), description: "Darkening spell".to_string() },
          ]
      }
  }
}

impl Read for Spellbook {
  fn list_spells(&self) -> Vec<String> {
      self.spells.iter().map(|x| x.name.to_owned()).collect()
  }

  fn find_spell(&self, name: String) -> Option<&Spell> {
      self.spells.iter().find(|e| e.name.to_lowercase().eq(name.to_lowercase().as_str()))
  }
}

impl Write for Spellbook {
  fn add_spell(&mut self, spell: Spell) -> bool {
      // let result = self.spells.iter().find(|e| e.name.to_lowercase().eq(spell.name.to_lowercase().as_str()));
      let result = self.find_spell(spell.name.to_owned());

      match result {
          Some(_) => false, // element exists...
          None => {
              self.spells.push(spell);
              true
          }
      }
  }

  fn remove_spell(&mut self, spell_name: String) -> bool {
      let result = self.spells.iter().position(|e| e.name.to_lowercase().eq(spell_name.to_lowercase().as_str()));
      // short version
      // self.spells.retain(|spell| spell.name != spell_name);

      match result {
          Some(index) => {
              self.spells.remove(index);
              true
          }
          None => false
      }
  }
}

fn main() {
  let mut spellbook = Spellbook::new();
  println!("{:?}", spellbook.list_spells());
  
  println!("\n-------\n");

  for spell_name in spellbook.list_spells() {
      match spellbook.find_spell(spell_name) {
          Some(spell) => println!("{}", spell),
          None => println!("unknown spell")
      }
  }    

  println!("\n-------\n");

  let add = spellbook.add_spell(Spell {
      name: "Leviosaa".to_string(),
      description: "es ist 'leviosa' und nicht 'leviosaaaaa'".to_string()
  });
  println!("added Leviosaa? {}", add);
  
  let add = spellbook.add_spell(Spell {
      name: "Lumos".to_string(),
      description: "mach halt des Licht an!".to_string()
  });
  println!("added Lumos? {}", add);

  println!("\n-------\n");

  let remove = spellbook.remove_spell("Avada Kedavra".to_string());
  println!("removed Avada Kedavra? {}", remove);

  println!("\n-------\n");

  match spellbook.find_spell("Leviosaa".to_string()) {
      Some(spell) => println!("{}", spell),
      None => println!("unknown spell")
  }

  println!("\n-------\n");

  let remove = spellbook.remove_spell("Leviosaa".to_string());
  println!("removed Leviosaa? {}", remove);

  match spellbook.find_spell("Leviosaa".to_string()) {
      Some(spell) => println!("{}", spell),
      None => println!("unknown spell: Leviosaa")
  }
}