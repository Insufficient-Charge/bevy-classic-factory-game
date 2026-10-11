use bevy::prelude::*;
use super::items::{ItemStack, ItemType};

/// Canvas satchel inventory holding up to `CAPACITY` item slots (40 slots per GDD Section 13.2 & 16.1).
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct Inventory {
    pub slots: Vec<Option<ItemStack>>,
    pub capacity: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new(40)
    }
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            slots: vec![None; capacity],
            capacity,
        }
    }

    /// Adds items to the inventory.
    /// Fills existing matching stacks first, then empty slots.
    /// Returns the number of items successfully stored.
    pub fn add_item(&mut self, item: ItemType, mut count: u32) -> u32 {
        let original_count = count;

        // 1. Fill existing matching stacks
        for slot in self.slots.iter_mut().flatten() {
            if slot.item == item && !slot.is_full() {
                let can_add = slot.remaining_capacity().min(count);
                slot.count += can_add;
                count -= can_add;
                if count == 0 {
                    return original_count;
                }
            }
        }

        // 2. Fill empty slots
        for slot in self.slots.iter_mut() {
            if slot.is_none() {
                let max = item.max_stack_size();
                let put = count.min(max);
                *slot = Some(ItemStack::new(item, put));
                count -= put;
                if count == 0 {
                    return original_count;
                }
            }
        }

        original_count - count
    }

    /// Checks how many items of a given type are in the inventory.
    pub fn count_item(&self, item: ItemType) -> u32 {
        self.slots
            .iter()
            .flatten()
            .filter(|s| s.item == item)
            .map(|s| s.count)
            .sum()
    }

    /// Returns true if the inventory contains at least `count` of `item`.
    pub fn has_item(&self, item: ItemType, count: u32) -> bool {
        self.count_item(item) >= count
    }

    /// Returns true if the inventory has all required items and quantities.
    pub fn has_items(&self, requirements: &[(ItemType, u32)]) -> bool {
        requirements
            .iter()
            .all(|&(item, needed)| self.has_item(item, needed))
    }

    /// Removes `count` of `item` from the inventory if available.
    /// Returns true if successfully removed, or false if insufficient quantity.
    pub fn remove_item(&mut self, item: ItemType, count: u32) -> bool {
        if !self.has_item(item, count) {
            return false;
        }

        let mut remaining_to_remove = count;
        for slot in self.slots.iter_mut() {
            if let Some(s) = slot {
                if s.item == item {
                    if s.count <= remaining_to_remove {
                        remaining_to_remove -= s.count;
                        *slot = None;
                    } else {
                        s.count -= remaining_to_remove;
                        remaining_to_remove = 0;
                    }

                    if remaining_to_remove == 0 {
                        break;
                    }
                }
            }
        }

        true
    }

    /// Consumes a list of item requirements from the inventory atomically.
    /// If any requirement cannot be satisfied, makes no changes and returns false.
    pub fn consume_items(&mut self, requirements: &[(ItemType, u32)]) -> bool {
        if !self.has_items(requirements) {
            return false;
        }

        for &(item, count) in requirements {
            assert!(self.remove_item(item, count));
        }

        true
    }

    /// Clears all slots.
    pub fn clear(&mut self) {
        for slot in self.slots.iter_mut() {
            *slot = None;
        }
    }
}

/// The tactile wooden hotbar (bottom 10 slots of the satchel per GDD Section 16.2).
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct Hotbar {
    pub selected_slot: usize,
    pub total_slots: usize,
    pub slots: [Option<ItemType>; 10],
}

impl Default for Hotbar {
    fn default() -> Self {
        Self {
            selected_slot: 0,
            total_slots: 10,
            slots: [None; 10],
        }
    }
}

impl Hotbar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn assign_slot(&mut self, slot: usize, item: ItemType) {
        if slot < self.total_slots {
            self.slots[slot] = Some(item);
        }
    }

    pub fn select_slot(&mut self, slot: usize) {
        if slot < self.total_slots {
            self.selected_slot = slot;
        }
    }

    pub fn next_slot(&mut self) {
        self.selected_slot = (self.selected_slot + 1) % self.total_slots;
    }

    pub fn prev_slot(&mut self) {
        self.selected_slot = (self.selected_slot + self.total_slots - 1) % self.total_slots;
    }

    /// Returns the item stack in the currently selected hotbar slot.
    pub fn get_selected<'a>(&self, inventory: &'a Inventory) -> Option<&'a ItemStack> {
        inventory.slots.get(self.selected_slot).and_then(|s| s.as_ref())
    }

    pub fn get_selected_item(&self) -> Option<ItemType> {
        self.slots[self.selected_slot]
    }
}

pub fn plugin(app: &mut App) {
    app.register_type::<Inventory>()
        .register_type::<Hotbar>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_add_and_count() {
        let mut inv = Inventory::new(5);
        let stored = inv.add_item(ItemType::IronOre, 50);
        assert_eq!(stored, 50);
        assert_eq!(inv.count_item(ItemType::IronOre), 50);

        // Add 60 more; max stack is 100, so 50 in first slot + 10 in second slot
        let stored2 = inv.add_item(ItemType::IronOre, 60);
        assert_eq!(stored2, 60);
        assert_eq!(inv.count_item(ItemType::IronOre), 110);
        assert_eq!(inv.slots[0].unwrap().count, 100);
        assert_eq!(inv.slots[1].unwrap().count, 10);
    }

    #[test]
    fn test_inventory_capacity_limit() {
        let mut inv = Inventory::new(1);
        let stored = inv.add_item(ItemType::IronOre, 150);
        // Only 100 fits in 1 slot
        assert_eq!(stored, 100);
        assert_eq!(inv.count_item(ItemType::IronOre), 100);
    }

    #[test]
    fn test_inventory_remove() {
        let mut inv = Inventory::new(5);
        inv.add_item(ItemType::IronOre, 80);
        assert!(inv.remove_item(ItemType::IronOre, 30));
        assert_eq!(inv.count_item(ItemType::IronOre), 50);

        // Cannot remove more than available
        assert!(!inv.remove_item(ItemType::IronOre, 60));
        assert_eq!(inv.count_item(ItemType::IronOre), 50);
    }

    #[test]
    fn test_consume_items_atomic() {
        let mut inv = Inventory::new(5);
        inv.add_item(ItemType::IronPlate, 4);
        inv.add_item(ItemType::BrassCog, 1);

        let req = [(ItemType::IronPlate, 4), (ItemType::BrassCog, 2)];
        // Fails because only 1 cog available; IronPlate must NOT be consumed!
        assert!(!inv.consume_items(&req));
        assert_eq!(inv.count_item(ItemType::IronPlate), 4);
        assert_eq!(inv.count_item(ItemType::BrassCog), 1);

        // Now provide enough
        inv.add_item(ItemType::BrassCog, 1);
        assert!(inv.consume_items(&req));
        assert_eq!(inv.count_item(ItemType::IronPlate), 0);
        assert_eq!(inv.count_item(ItemType::BrassCog), 0);
    }

    #[test]
    fn test_hotbar_cycling() {
        let mut hotbar = Hotbar::new();
        assert_eq!(hotbar.selected_slot, 0);

        hotbar.next_slot();
        assert_eq!(hotbar.selected_slot, 1);

        hotbar.prev_slot();
        assert_eq!(hotbar.selected_slot, 0);

        hotbar.prev_slot();
        assert_eq!(hotbar.selected_slot, 9);
    }
}
