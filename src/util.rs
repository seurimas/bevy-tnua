use bevy::prelude::*;
use bevy_tnua_physics_integration_layer::math::{
    AdjustPrecision, TargetFloat, TargetQuat, TargetVec2, TargetVec3,
};

pub struct SegmentedJumpInitialVelocityCalculator {
    height: TargetFloat,
    kinetic_energy: TargetFloat,
}

impl SegmentedJumpInitialVelocityCalculator {
    pub fn new(total_height: TargetFloat) -> Self {
        Self {
            height: total_height,
            kinetic_energy: 0.0,
        }
    }

    pub fn add_segment(
        &mut self,
        gravity: TargetFloat,
        velocity_threshold: TargetFloat,
    ) -> &mut Self {
        if self.height <= 0.0 {
            // No more height to jump
            return self;
        }

        let kinetic_energy_at_velocity_threshold = 0.5 * velocity_threshold.powi(2);

        let transferred_energy = kinetic_energy_at_velocity_threshold - self.kinetic_energy;
        if transferred_energy <= 0.0 {
            // Already faster than that velocity
            return self;
        }

        let segment_height = transferred_energy / gravity;
        if self.height < segment_height {
            // This segment will be the last
            self.kinetic_energy += self.height * gravity;
            self.height = 0.0;
        } else {
            self.kinetic_energy += transferred_energy;
            self.height -= segment_height;
        }

        self
    }

    pub fn kinetic_energy(&self) -> TargetFloat {
        self.kinetic_energy
    }
}

pub struct ProjectionPlaneForRotation {
    pub forward: TargetVec3,
    pub sideways: TargetVec3,
}

impl ProjectionPlaneForRotation {
    pub fn from_up_and_fowrard(up: Direction3d, forward: TargetVec3) -> Self {
        Self {
            forward,
            sideways: up.adjust_precision().cross(forward),
        }
    }

    pub fn from_up_using_default_forward(up: Direction3d) -> Self {
        Self::from_up_and_fowrard(up, TargetVec3::NEG_Z)
    }

    pub fn project_and_normalize(&self, vector: TargetVec3) -> TargetVec2 {
        TargetVec2::new(vector.dot(self.forward), vector.dot(self.sideways)).normalize_or_zero()
    }

    pub fn rotation_to_set_forward(
        &self,
        current_forward: TargetVec3,
        desired_forward: TargetVec3,
    ) -> TargetFloat {
        let rotation_to_set_forward = TargetQuat::from_rotation_arc_2d(
            self.project_and_normalize(current_forward),
            self.project_and_normalize(desired_forward),
        );
        rotation_to_set_forward.xyz().z
    }
}
