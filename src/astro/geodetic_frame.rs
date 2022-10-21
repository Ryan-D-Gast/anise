/*
 * ANISE Toolkit
 * Copyright (C) 2021-2022 Christopher Rabotin <christopher.rabotin@gmail.com> et al. (cf. AUTHORS.md)
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Documentation: https://nyxspace.com/
 */

use std::fmt::{Display, Formatter};

use crate::astro::RefFrame;
use crate::HashType;

/// A Frame uniquely defined by its ephemeris center and orientation.
///
/// # Notes
/// 1. If a frame defines a gravity parameter μ (mu), then it it considered a celestial object.
/// 2. If a frame defines an equatorial radius, a semi major radius, and a flattening ratio, then
/// is considered a geoid.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GeodeticRefFrame {
    pub frame: RefFrame,
    pub mu_km3s: f64,
    pub equatorial_radius: Option<f64>,
    pub semi_major_radius: Option<f64>,
    pub flattening: Option<f64>,
}

impl GeodeticRefFrame {
    /// Returns whether this frame is a geoid frame
    pub const fn is_geoid(&self) -> bool {
        self.equatorial_radius.is_some()
            && self.semi_major_radius.is_some()
            && self.flattening.is_some()
    }

    pub const fn ephem_origin_hash_match(&self, other_hash: HashType) -> bool {
        self.frame.ephem_origin_hash_match(other_hash)
    }

    pub const fn orient_origin_hash_match(&self, other_hash: HashType) -> bool {
        self.frame.orient_origin_hash_match(other_hash)
    }

    pub const fn ephem_origin_match(&self, other: Self) -> bool {
        self.frame.ephem_origin_match(other.frame)
    }

    pub const fn orient_origin_match(&self, other: Self) -> bool {
        self.frame.orient_origin_match(other.frame)
    }
}

impl Display for GeodeticRefFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.frame)?;
        write!(f, " (μ = {} km3/s", self.mu_km3s)?;

        if self.is_geoid() {
            write!(
                f,
                ", eq. radius = {} km, sm axis = {} km, f = {}",
                self.equatorial_radius.unwrap(),
                self.semi_major_radius.unwrap(),
                self.flattening.unwrap()
            )?;
        }
        write!(f, ")")
    }
}

#[allow(clippy::from_over_into)]
impl Into<RefFrame> for GeodeticRefFrame {
    /// Lossy operation to convert FrameDetail into a Frame.
    ///
    /// This will cause the LOSS of the constants stored in the frame detail.
    fn into(self) -> RefFrame {
        self.frame
    }
}
