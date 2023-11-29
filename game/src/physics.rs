use std::ops::Index;

use rapier2d::{prelude::*, na::Vector2};

use crate::engine::{Rect, Position};

pub struct World {
    gravity: Vector2<Real>,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    integration_parameters: IntegrationParameters,
    pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: Box<dyn PhysicsHooks>,
    event_handler: Box<dyn EventHandler>
}

impl Default for World {
    fn default() -> Self {
        World { 
            gravity: Vector2::new(0.0, 98.1), 
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            integration_parameters: IntegrationParameters::default(),
            pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(), 
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: Box::new(()),
            event_handler: Box::new(())
        }
    }
}

impl World {
    pub fn update(&mut self) {
        self.pipeline.step(
            &self.gravity, 
            &self.integration_parameters, 
            &mut self.island_manager, 
            &mut self.broad_phase, 
            &mut self.narrow_phase, 
            &mut self.rigid_body_set, 
            &mut self.collider_set, 
            &mut self.impulse_joint_set, 
            &mut self.multibody_joint_set, 
            &mut self.ccd_solver, 
            None, 
            self.physics_hooks.as_ref(), 
            self.event_handler.as_ref()
        );
    }

    pub fn add_collider(&mut self, rect: &Rect) -> ColliderHandle {
        let collider = ColliderBuilder::cuboid(
                (rect.width/2) as f32, (rect.height/2) as f32
            )
            .translation(
                vector![
                    (rect.x + rect.width/2) as f32, 
                    (rect.y + rect.height /2) as f32
                ]
            ).build();

        self.collider_set.insert(collider)
    }

    pub fn add_rigid_body(&mut self, rect: &Rect) -> RigidBodyHandle {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(
                vector![
                    (rect.x + rect.width/2) as f32, 
                    (rect.y + rect.height/2) as f32
                ]
            )
            .locked_axes(LockedAxes::TRANSLATION_LOCKED_X)
            .lock_rotations()
            .build();

        let collider = ColliderBuilder::cuboid(
            (rect.width as f32)/2.0, (rect.height as f32)/2.0
        )
            .restitution(0.0)
            .build();
        let body_handle = self.rigid_body_set.insert(rigid_body);
        self.collider_set.insert_with_parent(collider, body_handle, &mut self.rigid_body_set);
        body_handle
    }

    pub fn get_body_position(&self, handle: &RigidBodyHandle) -> Position {
        let body = &self.rigid_body_set[*handle];
        Position { x: body.translation().x as f64, y: body.translation().y as f64 }
    }

    pub fn add_impulse(&mut self, handle: &RigidBodyHandle, impulse: f32) {
        self
            .rigid_body_set
            .get_mut(*handle)
            .unwrap()
            .apply_impulse(vector![0.0, impulse], true);
    }
}
