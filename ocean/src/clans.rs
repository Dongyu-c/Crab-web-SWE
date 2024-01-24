use std::collections::HashMap;
#[derive(Debug)]
pub struct ClanSystem {
    // TODO: add necessary fields
    pub clan_id_map: HashMap<String,Vec<String>>,
}

impl ClanSystem {
    pub fn new() -> ClanSystem {

        ClanSystem{clan_id_map: HashMap::new()}
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        let out = self.clan_id_map.get(clan_id);
        if out.is_none(){
            Vec::new()
        }else{
            out.expect("REASON").to_vec()
        }
    }

    /**
     * Returns the number of clans currently in existence.
     */
    pub fn get_clan_count(&self) -> usize {
        self.clan_id_map.len()
        
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        let out = self.clan_id_map.get(clan_id);
        //println!("{}",clan_id.to_string());
        for i in out{
            println!("{}",i.get(0).expect("REASON").to_string());
        }
        if out.is_none(){
            0 as usize
        }else{
            out.expect("REASON").to_vec().len()
        }
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        if self.clan_id_map.len() == 0{
            None
        }else{
            let mut count:u32 = u32::MIN;
            let mut out = "Empty";
            for key in self.clan_id_map.keys(){
                //println!("{},{}",key,self.get_clan_member_count(key) as u32);
                if self.get_clan_member_count(key) as u32 > count{
                    count = self.get_clan_member_count(key) as u32;
                    out = key ;
                }
            }
            Some(out.to_string())
        }
    }
}
