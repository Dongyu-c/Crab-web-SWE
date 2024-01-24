use crate::color::Color;
use crate::cookbook::{Cookbook, Recipe};
use crate::diet::Diet;
use crate::prey::Prey;
use crate::reef::Reef;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Crab {
    // TODO: Add fields here (some in part 1, some in part 2)
    pub name: String,
    pub speed: u32,
    pub color: Color,
    pub diet: Diet,
    pub reefs: Vec<Rc<RefCell<Reef>>>,

}


// Do NOT implement Copy for Crab.
impl Crab {
    pub fn new(name: String, speed: u32, color: Color, diet: Diet) -> Crab {
        Crab { name, speed, color, diet , reefs:Vec::new()}
    }


    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn speed(&self) -> u32 {
        self.speed
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn diet(&self) -> Diet {
        self.diet
    }

    // PART 2 BELOW
    // ------------

    /**
     * Have this crab discover a new reef, adding it to its list of reefs.
     */
    pub fn discover_reef(&mut self, reef: Rc<RefCell<Reef>>) {
        
        self.reefs.push(reef);
        //unimplemented!();
    }

    /**
     * Returns Some prey from one of the reefs this crab feeds from,
     * and the index of that reef in self.reefs if able to find Some prey
     * using the `take_prey` method of Reef.
     *
     * If `take_prey` returns None, try the next reef. Try each reef only once.
     *
     * If all reefs are empty, or this crab has no reefs, return None.
     */
    fn catch_prey(&mut self) -> Option<(Box<dyn Prey>, usize)> {
        let mut out = None;
        if self.reefs.len() != 0{
            for i in 0..self.reefs.len(){
                if self.reefs.len() != 0{
                    let n_us: usize = i as usize;
                    let mut temp_reef = self.reefs[i].borrow_mut();

                    let mut old_out = temp_reef.take_prey();
                    if !old_out.is_none(){
                        out = Some((old_out.unwrap(),n_us));
                        return out;
                    }
                    
                }  
            }
        }
        return out;
    }

    /**
     * Releases the given prey back into the reef at the given index.
     */
    fn release_prey(&mut self, prey: Box<dyn Prey>, reef_index: usize) {
        let mut curr_reet = self.reefs[reef_index].borrow_mut();
        curr_reet.add_prey(prey);
    }

    /**
     * Have this crab go hunting.
     *
     * A crab will keep trying to catch prey until it succeeds,
     * or runs out of remaining prey to try to catch.
     *
     * You should keep track of all escaped prey in a local.
     *
     * Once you have finished hunting, release all escaped prey back into
     * the reefs from whence they came _before_ you return if prey was caught.
     *
     * Your algorithm might look something like this pseudocode. The challenge
     * in this task is not intended to be in figuring out the algorithm, but
     * in figuring out _how to write it in Rust_ using what you've learned so far.
     *
     * ```text
     *     there are no escaped prey yet
     *     prey has not been caught
     *     while prey can be caught
     *       if prey escapes
     *         mark the prey as escaped
     *         try again
     *     
     *       if prey is not edible by this crab
     *         mark the prey as escaped
     *         try again
     *       
     *       prey has been caught
     *       stop trying
     *     
     *     release each escaped prey back to its reef
     *     was prey caught?
     * ```
     *
     * Note: this pseudocode reads like a terrible poem.
     */
    pub fn hunt(&mut self) -> bool {
        // unimplemented!();
        //let mut escaped: Vec<(Box(dyn Prey),usize)> = Vec::new();
        let mut escaped: Vec<(Box<dyn Prey>,usize)> = Vec::new();
        let mut have = true;
        while 1 == 1{
            let mut old_prey_condition = self.catch_prey();
            
            if old_prey_condition.is_none(){
                have = false;
                break;
            }
            let mut prey_condition = old_prey_condition.unwrap();
            let mut prey = prey_condition.0;
            let mut loc = prey_condition.1;
    
            if prey.try_escape(&self){
                escaped.push( (prey, loc));
            }else {
                if prey.diet() != self.diet{
                    escaped.push( (prey, loc));
                }else{
                    break;
                }

            }

        }
        
        for i in escaped{
            let mut prey = i.0;
            let mut loc = i.1;
            self.release_prey( prey, loc);
            println!("RETURNED 111111111!!!!");
        }

        have

    }

    /**
     * Returns Some of any recipe from the given cookbook that matches the crab's diet
     * preferences, or None if no such recipe exists.
     *
     * IMPORTANT: you will need to add lifetime parameters to this function. It is
     * up to you to figure out which ones and where. Do not make any other changes
     * to the signature.
     */
    pub fn choose_recipe<'a>(&'a self, cookbook: &'a Cookbook) -> Option<&Recipe> {
        // if self.diet == Diet::Fish {
        //     println!("Fish!");
        // }else if self.diet == Diet::Shellfish{
        //     println!("Shellfish!");
        // }else {
        //     println!("Plants!");
        // }
        let mut out = None;
        for i in cookbook.recipes(){
            if i.diet() == self.diet {
                out = Some(i);
                
            }
        }
        
        return out;
        
    }
}
