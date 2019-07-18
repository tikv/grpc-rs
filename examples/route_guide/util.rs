// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unknown_lints)]
// client and server share different parts of utils.
#![allow(dead_code)]
#![allow(cast_lossless)]

use std::f64::consts::PI;

use serde_json;

use grpcio_proto::example::route_guide::*;

#[derive(Serialize, Deserialize, Debug)]
struct PointRef {
    latitude: i32,
    longitude: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct FeatureRef {
    location: PointRef,
    name: String,
}

impl From<FeatureRef> for Feature {
    fn from(r: FeatureRef) -> Feature {
        let mut f = Feature::default();
        f.set_name(r.name);
        f.mut_location().set_latitude(r.location.latitude);
        f.mut_location().set_longitude(r.location.longitude);
        f
    }
}

pub fn load_db() -> Vec<Feature> {
    let data = include_str!("db.json");
    let features: Vec<FeatureRef> = serde_json::from_str(data).unwrap();
    features.into_iter().map(From::from).collect()
}

pub fn same_point(lhs: &Point, rhs: &Point) -> bool {
    lhs.get_longitude() == rhs.get_longitude() && lhs.get_latitude() == rhs.get_latitude()
}

pub fn fit_in(lhs: &Point, rhs: &Rectangle) -> bool {
    let hi = rhs.get_hi();
    let lo = rhs.get_lo();
    lhs.get_longitude() <= hi.get_longitude()
        && lhs.get_longitude() >= lo.get_longitude()
        && lhs.get_latitude() <= hi.get_latitude()
        && lhs.get_latitude() >= lo.get_latitude()
}

const COORD_FACTOR: f64 = 10000000.0;

pub fn convert_to_rad(num: f64) -> f64 {
    num * PI / 180.0
}

pub fn format_point(p: &Point) -> String {
    format!(
        "{}, {}",
        p.get_latitude() as f64 / COORD_FACTOR,
        p.get_longitude() as f64 / COORD_FACTOR
    )
}

pub fn cal_distance(lhs: &Point, rhs: &Point) -> f64 {
    let lat1 = lhs.get_latitude() as f64 / COORD_FACTOR;
    let lon1 = lhs.get_longitude() as f64 / COORD_FACTOR;
    let lat2 = rhs.get_latitude() as f64 / COORD_FACTOR;
    let lon2 = rhs.get_longitude() as f64 / COORD_FACTOR;
    let lat_rad_1 = convert_to_rad(lat1);
    let lat_rad_2 = convert_to_rad(lat2);
    let delta_lat_rad = convert_to_rad(lat2 - lat1);
    let delta_lon_rad = convert_to_rad(lon2 - lon1);

    let a = (delta_lat_rad / 2.0).sin().powi(2)
        + lat_rad_1.cos() * lat_rad_2.cos() * (delta_lon_rad / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    let r = 6371000.0; // metres

    r * c
}
