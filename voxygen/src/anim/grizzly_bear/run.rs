use super::{
    super::{Animation, SkeletonAttr},
    GrizzlyBearSkeleton,
};
use std::{f32::consts::PI, ops::Mul};
use vek::*;

pub struct RunAnimation;

impl Animation for RunAnimation {
    type Skeleton = GrizzlyBearSkeleton;
    type Dependency = (f32, f64);

    fn update_skeleton(
        skeleton: &Self::Skeleton,
        (_velocity, global_time): Self::Dependency,
        anim_time: f64,
        _skeleton_attr: &SkeletonAttr,
    ) -> Self::Skeleton {
        let mut next = (*skeleton).clone();

        let wave = (anim_time as f32 * 14.0).sin();
        let wave_ultra_slow = (anim_time as f32 * 1.0 + PI).sin();
        let wave_ultra_slow_cos = (anim_time as f32 * 1.0 + PI).cos();
        let wave_slow = (anim_time as f32 * 3.5 + PI).sin();
        let wave_slow_cos = (anim_time as f32 * 3.5 + PI).cos();
        let wave_quick = (anim_time as f32 * 18.0).sin();
        let wave_med = (anim_time as f32 * 12.0).sin();
        let wave_med_cos = (anim_time as f32 * 12.0).cos();
        let wave_quick_cos = (anim_time as f32 * 18.0).cos();

         let grizzly_bear_look = Vec2::new(
            ((global_time + anim_time) as f32 / 8.0)
                .floor()
                .mul(7331.0)
                .sin()
                * 0.12,
            ((global_time + anim_time) as f32 / 8.0)
                .floor()
                .mul(1337.0)
                .sin()
                * 0.06,
        );

        next.grizzly_bear_upper_head.offset = Vec3::new(0.0, 12.0, 16.0 + wave_med * 0.15) / 8.46;
        next.grizzly_bear_upper_head.ori = Quaternion::rotation_x(wave_quick * 0.08 + grizzly_bear_look.y) * Quaternion::rotation_x(grizzly_bear_look.x);
        next.grizzly_bear_upper_head.scale = Vec3::one() / 8.46;

        next.grizzly_bear_lower_head.offset = Vec3::new(0.0, 9.5, -4.5 + wave_med * 0.15);
        next.grizzly_bear_lower_head.ori = Quaternion::rotation_x(wave_quick * 0.08);
        next.grizzly_bear_lower_head.scale = Vec3::one();

        next.grizzly_bear_upper_torso.offset = Vec3::new(-0.7, 0.5, 1.6 + wave_med * 0.15);
        next.grizzly_bear_upper_torso.ori = Quaternion::rotation_x(wave_quick * 0.08);
        next.grizzly_bear_upper_torso.scale = Vec3::one() / 8.46;

        next.grizzly_bear_lower_torso.offset = Vec3::new(-0.7, -1.1, 1.6 + wave_med * 0.15);
        next.grizzly_bear_lower_torso.ori = Quaternion::rotation_x(wave_quick * 0.08);
        next.grizzly_bear_lower_torso.scale = Vec3::one() / 8.46;

        next.grizzly_bear_leg_lf.offset = Vec3::new(1.0, 0.8, 2.0 + wave_med * 0.25);
        next.grizzly_bear_leg_lf.ori = Quaternion::rotation_x(wave_quick * 0.15);
        next.grizzly_bear_leg_lf.scale = Vec3::one() / 8.46;

        next.grizzly_bear_leg_rf.offset = Vec3::new(-1.0, 0.8, 2.0 + wave_med * 0.25);
        next.grizzly_bear_leg_rf.ori = Quaternion::rotation_x(wave_quick * 0.15);
        next.grizzly_bear_leg_rf.scale = Vec3::one() / 8.46;

        next.grizzly_bear_leg_lb.offset = Vec3::new(0.7, -1.6, 1.7 + wave_med * 0.25);
        next.grizzly_bear_leg_lb.ori = Quaternion::rotation_x(wave_quick * 0.15);
        next.grizzly_bear_leg_lb.scale = Vec3::one() / 8.46;

        next.grizzly_bear_leg_rb.offset = Vec3::new(-0.7, -1.6, 1.7 + wave_med * 0.25);
        next.grizzly_bear_leg_rb.ori = Quaternion::rotation_x(wave_quick * 0.15);
        next.grizzly_bear_leg_rb.scale = Vec3::one() / 8.46;

        next.grizzly_bear_foot_lf.offset = Vec3::new(-1.0, 2.0, -9.5 + wave_quick * 3.0 + wave_quick_cos * 3.0);
        next.grizzly_bear_foot_lf.ori = Quaternion::rotation_x(0.0 + wave_quick_cos * 0.55);
        next.grizzly_bear_foot_rb.scale = Vec3::one();

        next.grizzly_bear_foot_rf.offset = Vec3::new(1.0, 2.0, -9.5 - wave_quick * 3.0 + wave_quick_cos * 3.0);
        next.grizzly_bear_foot_rf.ori = Quaternion::rotation_x(0.0 + wave_quick_cos * 0.55);
        next.grizzly_bear_foot_rb.scale = Vec3::one();

        next.grizzly_bear_foot_lb.offset = Vec3::new(-1.0, 0.0, -6.5 - wave_quick * 3.0 + wave_quick_cos * 3.0);
        next.grizzly_bear_foot_lb.ori = Quaternion::rotation_x(0.0 + wave_quick_cos * 0.55);
        next.grizzly_bear_foot_rb.scale = Vec3::one();

        next.grizzly_bear_foot_rb.offset = Vec3::new(1.0, 0.0, -6.5 + wave_quick * 3.0 + wave_quick_cos * 3.0);
        next.grizzly_bear_foot_rb.ori = Quaternion::rotation_x(0.0 + wave_quick_cos * 0.55);
        next.grizzly_bear_foot_rb.scale = Vec3::one();

        next
    }
}
