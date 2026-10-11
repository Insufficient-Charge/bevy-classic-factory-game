use bevy::prelude::*;
use super::grid::GridCoord;
use super::inventory::Inventory;
use super::items::ItemType;

/// A harvestable resource vein or crystal deposit located on a tile.
#[derive(Component, Clone, Copy, Debug, Reflect)]
#[reflect(Component)]
pub struct ResourceDeposit {
    pub coord: GridCoord,
    pub item_type: ItemType,
    pub remaining_amount: u32,
    pub infinite: bool,
}

impl ResourceDeposit {
    pub const fn new(coord: GridCoord, item_type: ItemType, amount: u32) -> Self {
        Self {
            coord,
            item_type,
            remaining_amount: amount,
            infinite: false,
        }
    }

    pub const fn infinite(coord: GridCoord, item_type: ItemType) -> Self {
        Self {
            coord,
            item_type,
            remaining_amount: u32::MAX,
            infinite: true,
        }
    }

    /// Attempts to extract `amount` from this deposit.
    /// Returns the actual amount extracted.
    pub fn extract(&mut self, amount: u32) -> u32 {
        if self.infinite {
            return amount;
        }

        let extracted = self.remaining_amount.min(amount);
        self.remaining_amount -= extracted;
        extracted
    }

    pub fn is_depleted(&self) -> bool {
        !self.infinite && self.remaining_amount == 0
    }
}

/// The hand-cranked brass winding drill tool carried by The Constructor.
#[derive(Component, Clone, Copy, Debug, Reflect)]
#[reflect(Component)]
pub struct MiningTool {
    pub reach: f32,
    pub mining_rate: f32,
}

impl Default for MiningTool {
    fn default() -> Self {
        Self {
            reach: 2.5,
            mining_rate: 1.0,
        }
    }
}

/// Event requesting to hand-mine a tile.
#[derive(Event, Clone, Debug)]
pub struct MineTileEvent {
    pub player_entity: Entity,
    pub coord: GridCoord,
}

/// Event emitted when a tile is successfully mined.
#[derive(Event, Clone, Debug)]
pub struct TileMinedEvent {
    pub coord: GridCoord,
    pub item: ItemType,
    pub amount: u32,
}

/// Observer that handles `MineTileEvent`.
pub fn on_mine_tile(
    on: On<MineTileEvent>,
    players: Query<(&Transform, &MiningTool)>,
    mut inventories: Query<&mut Inventory>,
    mut deposits: Query<(Entity, &mut ResourceDeposit)>,
    mut commands: Commands,
) {
    let ev = on.event();
    let Ok((player_transform, tool)) = players.get(ev.player_entity) else {
        return;
    };

    let tile_world_pos = ev.coord.to_world_pos();
    let distance = player_transform.translation.distance(tile_world_pos);

    if distance > tool.reach {
        trace!("Target tile {:?} is out of reach ({:.2} > {:.2})", ev.coord, distance, tool.reach);
        return;
    }

    let Ok(mut inventory) = inventories.get_mut(ev.player_entity) else {
        return;
    };

    for (dep_entity, mut deposit) in deposits.iter_mut() {
        if deposit.coord == ev.coord && !deposit.is_depleted() {
            let extracted = deposit.extract(1);
            if extracted > 0 {
                let item = deposit.item_type;
                inventory.add_item(item, extracted);
                commands.trigger(TileMinedEvent {
                    coord: ev.coord,
                    item,
                    amount: extracted,
                });

                if deposit.is_depleted() {
                    commands.entity(dep_entity).despawn();
                }
            }
            break;
        }
    }
}

pub fn plugin(app: &mut App) {
    app.register_type::<ResourceDeposit>()
        .register_type::<MiningTool>()
        .add_observer(on_mine_tile);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_deposit_extraction() {
        let mut dep = ResourceDeposit::new(GridCoord::ground(0, 0), ItemType::IronOre, 5);
        assert_eq!(dep.extract(2), 2);
        assert_eq!(dep.remaining_amount, 3);
        assert!(!dep.is_depleted());

        assert_eq!(dep.extract(4), 3);
        assert_eq!(dep.remaining_amount, 0);
        assert!(dep.is_depleted());
    }

    #[test]
    fn test_infinite_deposit() {
        let mut dep = ResourceDeposit::infinite(GridCoord::ground(0, 0), ItemType::CopperOre);
        assert_eq!(dep.extract(100), 100);
        assert!(!dep.is_depleted());
    }

    #[test]
    fn test_hand_mining_reach_and_extraction() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_observer(on_mine_tile);

        // Player at origin with reach 2.5
        let player = app.world_mut().spawn((
            Transform::from_xyz(0.0, 0.0, 0.0),
            MiningTool { reach: 2.5, mining_rate: 1.0 },
            Inventory::default(),
        )).id();

        // Deposit within reach at (1, 1) -> distance sqrt(2) ≈ 1.41
        let dep_near = app.world_mut().spawn(
            ResourceDeposit::new(GridCoord::ground(1, 1), ItemType::IronOre, 3)
        ).id();

        // Deposit out of reach at (10, 10) -> distance sqrt(200) ≈ 14.14
        let dep_far = app.world_mut().spawn(
            ResourceDeposit::new(GridCoord::ground(10, 10), ItemType::CopperOre, 3)
        ).id();

        // Mine near deposit
        app.world_mut().commands().trigger(MineTileEvent {
            player_entity: player,
            coord: GridCoord::ground(1, 1),
        });
        app.update();

        // Assert player inventory received 1 IronOre
        let inv = app.world().get::<Inventory>(player).unwrap();
        assert_eq!(inv.count_item(ItemType::IronOre), 1);
        let dep = app.world().get::<ResourceDeposit>(dep_near).unwrap();
        assert_eq!(dep.remaining_amount, 2);

        // Attempt to mine far deposit
        app.world_mut().commands().trigger(MineTileEvent {
            player_entity: player,
            coord: GridCoord::ground(10, 10),
        });
        app.update();

        // Assert far deposit was NOT mined
        let inv = app.world().get::<Inventory>(player).unwrap();
        assert_eq!(inv.count_item(ItemType::CopperOre), 0);
        let dep = app.world().get::<ResourceDeposit>(dep_far).unwrap();
        assert_eq!(dep.remaining_amount, 3);
    }
}
