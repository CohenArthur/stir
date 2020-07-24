//! A `Recipe` is a collection of blocks. Use it to build your program
//! and run passes on it. You can also execute code from a `Recipe`.

use std::collections::HashMap;

use crate::blocks::BasicBlock;

/// BasicBlock collection
pub struct Recipe<'block> {
    entry: Option<&'block dyn BasicBlock>,
    blocks: HashMap<&'block String, &'block dyn BasicBlock>,
}

impl<'block> Recipe<'block> {
    /// Init a new Recipe
    pub fn new() -> Recipe<'block> {
        Recipe {
            entry: None,
            blocks: HashMap::new(),
        }
    }

    /// Add a block to the recipe. The first block you add is typically
    /// the entry point.
    ///
    /// ```
    /// use stir::blocks::Boolean;
    /// use stir::recipe::Recipe;
    ///
    /// let b = Boolean::new(false);
    ///
    /// let mut recipe = Recipe::new();
    /// recipe.add(&b);
    ///
    // FIXME: Content: add assert!(recipe.contains(b.label()));
    /// ```
    pub fn add(&mut self, block: &'block dyn BasicBlock) -> &Recipe {
        self.blocks.insert(block.label(), block);

        self
    }

    /// Set the entry point of the recipe
    // FIXME: Content: Add good example as it's an important function
    pub fn add_entry(&mut self, entry: &'block dyn BasicBlock) -> bool {
        match self.entry {
            None => {
                self.entry = Some(entry);
                self.add(entry);
                true
            }
            Some(_) => false,
        }
    }

    /// Interpret and execute the recipe
    pub fn fry(&self) -> Result<bool, ()> {
        match self.entry {
            Some(entry_block) => Ok(BasicBlock::interpret(entry_block)),
            None => Err(()),
        }
    }

    /// Return the entry point of the Recipe
    // FIXME: Content: Add good example as it's an important function
    pub fn entry(&self) -> Option<&'block dyn BasicBlock> {
        self.entry
    }

    /// Return the number of blocks in the Recipe
    pub fn len(&self) -> usize {
        self.blocks.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::blocks::Boolean;

    #[test]
    fn init() {
        let r = Recipe::new();

        match r.entry() {
            Some(_) => assert!(false),
            _ => (),
        };

        assert_eq!(r.len(), 0);
    }

    #[test]
    fn add_one_block_size_and_entry() {
        let mut r = Recipe::new();
        let b = Boolean::new(false);

        r.add(&b);

        match r.entry() {
            Some(_) => assert!(false),
            _ => (),
        };

        assert_eq!(r.len(), 1);
    }

    #[test]
    fn add_entry() {
        let mut r = Recipe::new();
        let b = Boolean::new(false);

        r.add_entry(&b);

        match r.entry() {
            None => assert!(false),
            _ => (),
        };

        assert_eq!(r.len(), 1);
    }
}
