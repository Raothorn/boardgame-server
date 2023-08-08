use std::collections::HashMap;

pub struct Map {
    ship_location: RegionIx
}

pub struct MapData {
    regions: HashMap<RegionIx, Region>,
    region_graph: HashMap<RegionIx, Vec<RegionIx>>,
}

pub struct Region {
    ports: Vec<PortIx>
}

pub struct Port {

}

type RegionIx = u32;
type PortIx = u32;
