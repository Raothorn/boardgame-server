use std::collections::HashMap;

use iter_tools::Itertools;
use serde::Serialize;

use super::storybook::Storybook;

#[derive(Clone)]
pub struct GameMap {
    pub ship_area: AreaIx,
    pub map_data: MapData,
    pub storybook: Storybook
}

impl GameMap {
    fn current_region(&self) -> RegionIx {
        self.map_data
            .areas
            .get(&self.ship_area)
            .map(|a| a.region)
            .unwrap() // We want to panic here, our graph is incomplete
    }
}

#[derive(Clone)]
pub struct MapData {
    areas: HashMap<AreaIx, Area>,
    area_graph: HashMap<AreaIx, Vec<AreaIx>>,
}

impl MapData {
    fn areas_in_region(&self, region: RegionIx) -> Vec<AreaIx> {
        self.areas
            .iter()
            .filter(|(_, area)| area.region == region)
            .map(|(ix, _)| *ix)
            .collect()
    }

    fn adjacent_areas(&self, area: AreaIx) -> Vec<AreaIx> {
        self.area_graph[&area].clone()
    }
}

#[derive(Clone)]
struct Area {
    region: RegionIx,
    ports: Vec<PortIx>
}


type RegionIx = u32;
type AreaIx = u32;
type PortIx = u32;

#[derive(Clone, Serialize)]
pub struct SerialMap {
    pub ship_area: AreaIx,
    pub adjacent_areas: Vec<AreaIx>,
    pub adjacent_ports: Vec<PortIx>,
    pub visible_areas: Vec<AreaIx>,
    pub visible_ports: Vec<PortIx>,
    pub current_region: RegionIx,
}

impl From<GameMap> for SerialMap {
    fn from(map: GameMap) -> Self {
        let adjacent_areas = map.map_data.adjacent_areas(map.ship_area).clone();

        // The visible areas are all the areas in the region plus areas immediately adjacent
        let area_ixs = map.map_data.areas_in_region(map.current_region());

        let all_adjacent: Vec<AreaIx> = area_ixs
            .iter()
            .flat_map(|a| map.map_data.adjacent_areas(*a))
            .collect();

        let visible_areas = area_ixs
            .iter()
            .chain(all_adjacent.iter())
            .unique()
            .cloned()
            .collect();

        let adjacent_ports = map.map_data.areas[&map.ship_area].ports.clone();

        let areas = area_ixs.iter().map(|a_ix| map.map_data.areas[a_ix].clone());
        let visible_ports = areas.map(|a| a.ports).flatten().collect();

        SerialMap {
            ship_area: map.ship_area,
            adjacent_areas,
            adjacent_ports,
            visible_areas,
            visible_ports,
            current_region: map.current_region(),
        }
    }
}

// map data
impl Default for MapData {
    fn default() -> Self {
        let mut areas = HashMap::new();
        areas.insert(1, Area { region: 1, ports: vec![2, 174]});
        areas.insert(2, Area { region: 1, ports: vec![18, 34, 186] });
        areas.insert(3, Area { region: 1, ports: vec![7, 30, 216] });
        areas.insert(4, Area { region: 1, ports: vec![130] });

        areas.insert(6, Area { region: 2, ports: vec![] });
        areas.insert(7, Area { region: 2, ports: vec![] });

        let mut area_graph = HashMap::new();

        area_graph.insert(1, vec![2, 4, 6]);
        area_graph.insert(2, vec![1, 3]);
        area_graph.insert(3, vec![2, 4]);
        area_graph.insert(4, vec![3, 1]);
        area_graph.insert(6, vec![1, 7]);
        area_graph.insert(7, vec![6]);

        Self { areas, area_graph }
    }
}
