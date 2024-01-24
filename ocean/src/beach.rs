use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    // TODO: Declare the fields of the Beach struct here.
    crab_vec: Vec<Crab>,
    c_system: ClanSystem,
}

impl Beach {
    pub fn new() -> Beach {
        
        Beach {crab_vec:Vec::new(),c_system:ClanSystem::new()}
    }


    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.crab_vec.len()
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.crab_vec.push(crab);
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        self.crab_vec.get(index).expect("REASON")
    }

    pub fn crabs(&self) -> Iter<Crab> {
        self.crab_vec.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        if self.crab_vec.len() == 0 {
            None
        } else {
            let mut out = self.crab_vec.get(0).expect("REASON");
            for i in &self.crab_vec {
                if i.speed > out.speed{
                    out = &i;
                }
            }

            Some(out)
        } 
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        let mut out = Vec::new();
        for i in &self.crab_vec{
            if i.name == name{
                out.push(i);
            }
        }
        out
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        let color = Color::cross(&self.get_crab(i).color,&self.get_crab(j).color);
        let diet = Diet::random_diet();
        let speed = 1;
        let baby = Crab::new(name, speed, color, diet);
        let _ = &self.crab_vec.push(baby);
    }

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        &self.c_system
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        
        if !(self.c_system.clan_id_map.contains_key(&crab_name.to_string())){
            self.c_system.clan_id_map.entry(clan_id.to_string()).and_modify(|vec| vec.push(crab_name.to_string())) 
            .or_insert(vec![crab_name.to_string()]); 
        }
        
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        let clan1 = self.get_clan_system().get_clan_member_names(id1);
        let clan2 = self.get_clan_system().get_clan_member_names(id2);
        if clan1.len() == 0{
            return Err("The inputs are invalid.".to_string());
        }
        if clan2.len() == 0{
            return Err("The inputs are invalid.".to_string());
        }
        let mut a = 0;
        let mut b = 0;
        
        for i in &clan1{
            println!("{}",i);
            a = a + self.find_crabs_by_name(&i).get(0).expect("REASON").speed();
        }
        a = a/(clan1.len() as u32);

        for i in &clan2{
            println!("{}",i);
            b = b + self.find_crabs_by_name(&i).get(0).expect("REASON").speed();
        }
        b = b/(clan2.len() as u32);
        if a == b {
            return Ok(None);
        }
        if a > b {
            return Ok(Some(id1.to_string()));
        }
        if a < b {
            return Ok(Some(id2.to_string()));
        }
        unimplemented!();
        
    }
}
