use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VehicleKind {
    PlayerRed,
    CarSedanBlue,
    CarTaxiYellow,
    CarHatchbackGreen,
    CarPolice,
    TruckDelivery,
    LimoWhite,
    Ambulance,
    SemiTruck,
    BusTransit,
    #[serde(other)]
    Unknown,
}

impl VehicleKind {
    pub const fn sprite_name(&self, orientation: Orientation) -> &'static str {
        match (self, orientation) {
            (Self::PlayerRed, Orientation::Horizontal) => "player_red_h",
            (Self::PlayerRed, Orientation::Vertical) => "player_red_v",

            (Self::CarSedanBlue, Orientation::Horizontal) => "car_sedan_blue_h",
            (Self::CarSedanBlue, Orientation::Vertical) => "car_sedan_blue_v",

            (Self::CarTaxiYellow, Orientation::Horizontal) => "car_taxi_yellow_h",
            (Self::CarTaxiYellow, Orientation::Vertical) => "car_taxi_yellow_v",

            (Self::CarHatchbackGreen, Orientation::Horizontal) => "car_hatchback_green_h",
            (Self::CarHatchbackGreen, Orientation::Vertical) => "car_hatchback_green_v",

            (Self::CarPolice, Orientation::Horizontal) => "car_police_h",
            (Self::CarPolice, Orientation::Vertical) => "car_police_v",

            (Self::TruckDelivery, Orientation::Horizontal) => "truck_delivery_h",
            (Self::TruckDelivery, Orientation::Vertical) => "truck_delivery_v",

            (Self::LimoWhite, Orientation::Horizontal) => "limo_white_h",
            (Self::LimoWhite, Orientation::Vertical) => "limo_white_v",

            (Self::Ambulance, Orientation::Horizontal) => "ambulance_h",
            (Self::Ambulance, Orientation::Vertical) => "ambulance_v",

            (Self::SemiTruck, Orientation::Horizontal) => "semi_truck_h",
            (Self::SemiTruck, Orientation::Vertical) => "semi_truck_v",

            (Self::BusTransit, Orientation::Horizontal) => "bus_transit_h",
            (Self::BusTransit, Orientation::Vertical) => "bus_transit_v",

            (Self::Unknown, Orientation::Horizontal) => "player_red_h",
            (Self::Unknown, Orientation::Vertical) => "player_red_v",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: String,
    pub kind: VehicleKind,
    pub x: i32,
    pub y: i32,
    pub length: i32,
    pub orientation: Orientation,
    #[serde(default)]
    pub is_player: bool,
    #[serde(skip)]
    pub drag_offset: f32,
}

impl Vehicle {
    #[allow(dead_code)]
    pub fn new(
        id: impl Into<String>,
        kind: VehicleKind,
        x: i32,
        y: i32,
        length: i32,
        orientation: Orientation,
        is_player: bool,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            x,
            y,
            length,
            orientation,
            is_player,
            drag_offset: 0.0,
        }
    }

    /// Returns an iterator of grid coordinates occupied by this vehicle.
    pub fn occupied_cells(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        let (vx, vy, len, orient) = (self.x, self.y, self.length, self.orientation);
        (0..len).map(move |i| match orient {
            Orientation::Horizontal => (vx + i, vy),
            Orientation::Vertical => (vx, vy + i),
        })
    }

    /// Checks whether a given grid coordinate is occupied by this vehicle.
    pub fn contains_cell(&self, cell_x: i32, cell_y: i32) -> bool {
        self.occupied_cells()
            .any(|(cx, cy)| cx == cell_x && cy == cell_y)
    }

    /// Returns the pixel bounding box for rendering and touch hit testing.
    pub fn pixel_bounds(
        &self,
        origin_x: f32,
        origin_y: f32,
        cell_size: f32,
    ) -> (f32, f32, f32, f32) {
        let (px, py) = match self.orientation {
            Orientation::Horizontal => (
                origin_x + (self.x as f32 + self.drag_offset) * cell_size,
                origin_y + self.y as f32 * cell_size,
            ),
            Orientation::Vertical => (
                origin_x + self.x as f32 * cell_size,
                origin_y + (self.y as f32 + self.drag_offset) * cell_size,
            ),
        };

        let (w, h) = match self.orientation {
            Orientation::Horizontal => (self.length as f32 * cell_size, cell_size),
            Orientation::Vertical => (cell_size, self.length as f32 * cell_size),
        };

        (px, py, w, h)
    }
}
