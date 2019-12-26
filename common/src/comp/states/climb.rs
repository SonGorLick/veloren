use super::{
    ActionState::*, CharacterState, ECSStateData, ECSStateUpdate, FallHandler, JumpHandler,
    MoveState::*, StandHandler, StateHandle,
};
use super::{CLIMB_SPEED, HUMANOID_CLIMB_ACCEL, HUMANOID_SPEED};
use crate::sys::phys::GRAVITY;
use vek::vec::{Vec2, Vec3};
use vek::Lerp;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct ClimbHandler;

impl StateHandle for ClimbHandler {
    fn handle(&self, ecs_data: &ECSStateData) -> ECSStateUpdate {
        let mut update = ECSStateUpdate {
            pos: *ecs_data.pos,
            vel: *ecs_data.vel,
            ori: *ecs_data.ori,
            character: *ecs_data.character,
        };

        // Move player according to move_dir
        update.vel.0 += Vec2::broadcast(ecs_data.dt.0)
            * ecs_data.inputs.move_dir
            * if update.vel.0.magnitude_squared() < HUMANOID_SPEED.powf(2.0) {
                HUMANOID_CLIMB_ACCEL
            } else {
                0.0
            };

        // Set direction based on move direction when on the ground
        let ori_dir = if let Some(wall_dir) = ecs_data.physics.on_wall {
            if Vec2::<f32>::from(wall_dir).magnitude_squared() > 0.001 {
                Vec2::from(wall_dir).normalized()
            } else {
                Vec2::from(update.vel.0)
            }
        } else {
            Vec2::from(update.vel.0)
        };

        if ori_dir.magnitude_squared() > 0.0001
            && (update.ori.0.normalized() - Vec3::from(ori_dir).normalized()).magnitude_squared()
                > 0.001
        {
            update.ori.0 = vek::ops::Slerp::slerp(
                update.ori.0,
                ori_dir.into(),
                if ecs_data.physics.on_ground { 9.0 } else { 2.0 } * ecs_data.dt.0,
            );
        }

        // Apply Vertical Climbing Movement
        if let (true, Some(_wall_dir)) = (
            (ecs_data.inputs.climb.is_pressed() | ecs_data.inputs.climb_down.is_pressed())
                && update.vel.0.z <= CLIMB_SPEED,
            ecs_data.physics.on_wall,
        ) {
            if ecs_data.inputs.climb_down.is_pressed() && !ecs_data.inputs.climb.is_pressed() {
                update.vel.0 -=
                    ecs_data.dt.0 * update.vel.0.map(|e| e.abs().powf(1.5) * e.signum() * 6.0);
            } else if ecs_data.inputs.climb.is_pressed() && !ecs_data.inputs.climb_down.is_pressed()
            {
                update.vel.0.z = (update.vel.0.z + ecs_data.dt.0 * GRAVITY * 1.25).min(CLIMB_SPEED);
            } else {
                update.vel.0.z = update.vel.0.z + ecs_data.dt.0 * GRAVITY * 1.5;
                update.vel.0 = Lerp::lerp(
                    update.vel.0,
                    Vec3::zero(),
                    30.0 * ecs_data.dt.0 / (1.0 - update.vel.0.z.min(0.0) * 5.0),
                );
            }
        }

        if let None = ecs_data.physics.on_wall {
            if ecs_data.inputs.jump.is_pressed() {
                update.character = CharacterState {
                    action_state: Idle,
                    move_state: Jump(JumpHandler),
                };
                return update;
            } else {
                update.character = CharacterState {
                    action_state: Idle,
                    move_state: Fall(FallHandler),
                };
                return update;
            }
        }
        if ecs_data.physics.on_ground {
            update.character = CharacterState {
                action_state: Idle,
                move_state: Stand(StandHandler),
            };
            return update;
        }

        return update;
    }
}
