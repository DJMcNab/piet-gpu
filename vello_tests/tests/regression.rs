// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tests to ensure that certain issues which don't deserve a test scene don't regress

use vello::{
    kurbo::{Affine, Rect, RoundedRect, Stroke},
    peniko::color::palette,
    AaConfig, Scene,
};
use vello_tests::{snapshot_test_sync, TestParams};

/// Test created from <https://github.com/linebender/vello/issues/616>
#[test]
#[cfg_attr(skip_gpu_tests, ignore)]
fn rounded_rectangle_watertight() {
    let mut scene = Scene::new();
    let rect = RoundedRect::new(60.0, 10.0, 80.0, 30.0, 10.0);
    let stroke = Stroke::new(2.0);
    scene.stroke(&stroke, Affine::IDENTITY, palette::css::WHITE, None, &rect);
    let mut params = TestParams::new("rounded_rectangle_watertight", 70, 30);
    params.anti_aliasing = AaConfig::Msaa16;
    snapshot_test_sync(scene, &params)
        .unwrap()
        .assert_mean_less_than(0.001);
}

/// Test created from <https://github.com/linebender/vello/issues/662>
#[test]
#[cfg_attr(skip_gpu_tests, ignore)]
fn stroke_width_zero() {
    let mut scene = Scene::new();
    let stroke = Stroke::new(0.0);
    let rect = Rect::new(10.0, 10.0, 40.0, 40.0);
    let rect_stroke_color = palette::css::PEACH_PUFF;
    scene.stroke(&stroke, Affine::IDENTITY, rect_stroke_color, None, &rect);
    let mut params = TestParams::new("stroke_width_zero", 50, 50);
    params.anti_aliasing = AaConfig::Msaa16;
    snapshot_test_sync(scene, &params)
        .unwrap()
        .assert_mean_less_than(0.001);
}
