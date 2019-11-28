use crate::{ text };
use piet::TextLayout;

// - x: calculated value
// - target: f64
// - tolerance: in f64
fn assert_close_to(x: f64, target: f64, tolerance: f64) {
    let min = target - tolerance;
    let max = target + tolerance;
    println!("x: {}, target: {}", x, target);
    assert!(x <= max && x >= min);
}

#[test]
fn test_hit_test_text_position_basic() {
    let mut text_layout = text::EmbedText::new();

    let input = "piet text!";
    let font = text_layout
        .new_font_by_name("sans-serif", 12.0)
        .build()
        .unwrap();

    let layout = text_layout
        .new_text_layout(&font, &input[0..4])
        .build()
        .unwrap();
    let piet_width = layout.width();

    let layout = text_layout
        .new_text_layout(&font, &input[0..3])
        .build()
        .unwrap();
    let pie_width = layout.width();

    let layout = text_layout
        .new_text_layout(&font, &input[0..2])
        .build()
        .unwrap();
    let pi_width = layout.width();

    let layout = text_layout
        .new_text_layout(&font, &input[0..1])
        .build()
        .unwrap();
    let p_width = layout.width();

    let layout = text_layout.new_text_layout(&font, "").build().unwrap();
    let null_width = layout.width();

    let full_layout = text_layout.new_text_layout(&font, input).build().unwrap();
    let full_width = full_layout.width();

    assert_close_to(
        full_layout.hit_test_text_position(4).unwrap().point.x as f64,
        piet_width,
        3.0,
    );
    assert_close_to(
        full_layout.hit_test_text_position(3).unwrap().point.x as f64,
        pie_width,
        3.0,
    );
    assert_close_to(
        full_layout.hit_test_text_position(2).unwrap().point.x as f64,
        pi_width,
        3.0,
    );
    assert_close_to(
        full_layout.hit_test_text_position(1).unwrap().point.x as f64,
        p_width,
        3.0,
    );
    assert_close_to(
        full_layout.hit_test_text_position(0).unwrap().point.x as f64,
        null_width,
        3.0,
    );
    assert_close_to(
        full_layout.hit_test_text_position(10).unwrap().point.x as f64,
        full_width,
        3.0,
    );
    assert_close_to(
        full_layout.hit_test_text_position(11).unwrap().point.x as f64,
        full_width,
        3.0,
    );
    assert_eq!(
        full_layout
            .hit_test_text_position(11)
            .unwrap()
            .metrics
            .text_position,
        10
    )
}

#[test]
fn test_hit_test_text_position_complex_0() {
    let input = "é";
    assert_eq!(input.len(), 2);

    let mut text_layout = EmbedText::new();
    let font = text_layout
        .new_font_by_name("sans-serif", 12.0)
        .build()
        .unwrap();
    let layout = text_layout.new_text_layout(&font, input).build().unwrap();

    assert_close_to(layout.hit_test_text_position(0).unwrap().point.x, 0.0, 3.0);
    assert_close_to(
        layout.hit_test_text_position(2).unwrap().point.x,
        layout.width(),
        3.0,
    );

    // note code unit not at grapheme boundary
    // This one panics in d2d because this is not a code unit boundary.
    // But it works here! Harder to deal with this right now, since unicode-segmentation
    // doesn't give code point offsets.
    assert_close_to(layout.hit_test_text_position(1).unwrap().point.x, 0.0, 3.0);
    assert_eq!(
        layout
            .hit_test_text_position(1)
            .unwrap()
            .metrics
            .text_position,
        1
    );

    // unicode segmentation is wrong on this one for now.
    //let input = "🤦\u{1f3fc}\u{200d}\u{2642}\u{fe0f}";

    //let mut text_layout = D2DText::new();
    //let font = text_layout.new_font_by_name("sans-serif", 12.0).build().unwrap();
    //let layout = text_layout.new_text_layout(&font, input).build().unwrap();

    //assert_eq!(input.graphemes(true).count(), 1);
    //assert_eq!(layout.hit_test_text_position(0, true).map(|p| p.point_x as f64), Some(layout.width()));
    //assert_eq!(input.len(), 17);

    let input = "\u{0023}\u{FE0F}\u{20E3}"; // #️⃣
    assert_eq!(input.len(), 7);
    assert_eq!(input.chars().count(), 3);

    let mut text_layout = EmbedText::new();
    let font = text_layout
        .new_font_by_name("sans-serif", 12.0)
        .build()
        .unwrap();
    let layout = text_layout.new_text_layout(&font, input).build().unwrap();

    assert_close_to(layout.hit_test_text_position(0).unwrap().point.x, 0.0, 3.0);
    assert_close_to(
        layout.hit_test_text_position(7).unwrap().point.x,
        layout.width(),
        3.0,
    );

    // note code unit not at grapheme boundary
    assert_close_to(layout.hit_test_text_position(1).unwrap().point.x, 0.0, 3.0);
    assert_eq!(
        layout
            .hit_test_text_position(1)
            .unwrap()
            .metrics
            .text_position,
        1
    );
}

#[test]
fn test_hit_test_text_position_complex_1() {
    // Notes on this input:
    // 6 code points
    // 7 utf-16 code units (1/1/1/1/1/2)
    // 14 utf-8 code units (2/1/3/3/1/4)
    // 4 graphemes
    let input = "é\u{0023}\u{FE0F}\u{20E3}1\u{1D407}"; // #️⃣,, 𝐇
    assert_eq!(input.len(), 14);

    let mut text_layout = EmbedText::new();
    let font = text_layout
        .new_font_by_name("sans-serif", 12.0)
        .build()
        .unwrap();
    let layout = text_layout.new_text_layout(&font, input).build().unwrap();

    let test_layout_0 = text_layout
        .new_text_layout(&font, &input[0..2])
        .build()
        .unwrap();
    let test_layout_1 = text_layout
        .new_text_layout(&font, &input[0..9])
        .build()
        .unwrap();
    let test_layout_2 = text_layout
        .new_text_layout(&font, &input[0..10])
        .build()
        .unwrap();

    // Note: text position is in terms of utf8 code units
    assert_close_to(layout.hit_test_text_position(0).unwrap().point.x, 0.0, 3.0);
    assert_close_to(
        layout.hit_test_text_position(2).unwrap().point.x,
        test_layout_0.width(),
        3.0,
    );
    assert_close_to(
        layout.hit_test_text_position(9).unwrap().point.x,
        test_layout_1.width(),
        3.0,
    );
    assert_close_to(
        layout.hit_test_text_position(10).unwrap().point.x,
        test_layout_2.width(),
        3.0,
    );
    assert_close_to(
        layout.hit_test_text_position(14).unwrap().point.x,
        layout.width(),
        3.0,
    );

    // Code point boundaries, but not grapheme boundaries.
    // Width should stay at the current grapheme boundary.
    assert_close_to(
        layout.hit_test_text_position(3).unwrap().point.x,
        test_layout_0.width(),
        3.0,
    );
    assert_eq!(
        layout
            .hit_test_text_position(3)
            .unwrap()
            .metrics
            .text_position,
        3
    );
    assert_close_to(
        layout.hit_test_text_position(6).unwrap().point.x,
        test_layout_0.width(),
        3.0,
    );
    assert_eq!(
        layout
            .hit_test_text_position(6)
            .unwrap()
            .metrics
            .text_position,
        6
    );
}

#[test]
#[cfg(target_os = "linux")]
fn test_hit_test_point_basic_0() {
    let mut text_layout = EmbedText::new();

    let font = text_layout
        .new_font_by_name("sans-serif", 12.0)
        .build()
        .unwrap();
    let layout = text_layout
        .new_text_layout(&font, "piet text!")
        .build()
        .unwrap();
    println!("text pos 4: {:?}", layout.hit_test_text_position(4)); // 23.0
    println!("text pos 5: {:?}", layout.hit_test_text_position(5)); // 27.0

    // test hit test point
    // all inside
    let pt = layout.hit_test_point(Point::new(22.5, 0.0));
    assert_eq!(pt.metrics.text_position, 4);
    let pt = layout.hit_test_point(Point::new(23.0, 0.0));
    assert_eq!(pt.metrics.text_position, 4);
    let pt = layout.hit_test_point(Point::new(25.0, 0.0));
    assert_eq!(pt.metrics.text_position, 5);
    let pt = layout.hit_test_point(Point::new(26.0, 0.0));
    assert_eq!(pt.metrics.text_position, 5);
    let pt = layout.hit_test_point(Point::new(27.0, 0.0));
    assert_eq!(pt.metrics.text_position, 5);
    let pt = layout.hit_test_point(Point::new(28.0, 0.0));
    assert_eq!(pt.metrics.text_position, 5);

    // outside
    println!("layout_width: {:?}", layout.width()); // 56.0

    let pt = layout.hit_test_point(Point::new(56.0, 0.0));
    assert_eq!(pt.metrics.text_position, 10); // last text position
    assert_eq!(pt.is_inside, true);

    let pt = layout.hit_test_point(Point::new(57.0, 0.0));
    assert_eq!(pt.metrics.text_position, 10); // last text position
    assert_eq!(pt.is_inside, false);

    let pt = layout.hit_test_point(Point::new(-1.0, 0.0));
    assert_eq!(pt.metrics.text_position, 0); // first text position
    assert_eq!(pt.is_inside, false);
}

#[test]
#[cfg(target_os = "macos")]
fn test_hit_test_point_basic_0() {
    let mut text_layout = EmbedText::new();

    let font = text_layout
        .new_font_by_name("sans-serif", 12.0)
        .build()
        .unwrap();
    let layout = text_layout
        .new_text_layout(&font, "piet text!")
        .build()
        .unwrap();
    println!("text pos 4: {:?}", layout.hit_test_text_position(4)); // 19.34765625
    println!("text pos 5: {:?}", layout.hit_test_text_position(5)); // 22.681640625

    // test hit test point
    // all inside
    let pt = layout.hit_test_point(Point::new(19.0, 0.0));
    assert_eq!(pt.metrics.text_position, 4);
    let pt = layout.hit_test_point(Point::new(20.0, 0.0));
    assert_eq!(pt.metrics.text_position, 4);
    let pt = layout.hit_test_point(Point::new(21.0, 0.0));
    assert_eq!(pt.metrics.text_position, 4);
    let pt = layout.hit_test_point(Point::new(22.0, 0.0));
    assert_eq!(pt.metrics.text_position, 5);
    let pt = layout.hit_test_point(Point::new(23.0, 0.0));
    assert_eq!(pt.metrics.text_position, 5);

    // outside
    println!("layout_width: {:?}", layout.width()); //45.357421875

    let pt = layout.hit_test_point(Point::new(45.0, 0.0));
    assert_eq!(pt.metrics.text_position, 10); // last text position
    assert_eq!(pt.is_inside, true);

    let pt = layout.hit_test_point(Point::new(46.0, 0.0));
    assert_eq!(pt.metrics.text_position, 10); // last text position
    assert_eq!(pt.is_inside, false);

    let pt = layout.hit_test_point(Point::new(-1.0, 0.0));
    assert_eq!(pt.metrics.text_position, 0); // first text position
    assert_eq!(pt.is_inside, false);
}

#[test]
#[cfg(target_os = "linux")]
// for testing that 'middle' assignment in binary search is correct
fn test_hit_test_point_basic_1() {
    let mut text_layout = EmbedText::new();

    // base condition, one grapheme
    let font = text_layout
        .new_font_by_name("sans-serif", 12.0)
        .build()
        .unwrap();
    let layout = text_layout.new_text_layout(&font, "t").build().unwrap();
    println!("text pos 1: {:?}", layout.hit_test_text_position(1)); // 5.0

    // two graphemes (to check that middle moves)
    let pt = layout.hit_test_point(Point::new(1.0, 0.0));
    assert_eq!(pt.metrics.text_position, 0);

    let layout = text_layout.new_text_layout(&font, "te").build().unwrap();
    println!("text pos 1: {:?}", layout.hit_test_text_position(1)); // 5.0
    println!("text pos 2: {:?}", layout.hit_test_text_position(2)); // 12.0

    let pt = layout.hit_test_point(Point::new(1.0, 0.0));
    assert_eq!(pt.metrics.text_position, 0);
    let pt = layout.hit_test_point(Point::new(4.0, 0.0));
    assert_eq!(pt.metrics.text_position, 1);
    let pt = layout.hit_test_point(Point::new(6.0, 0.0));
    assert_eq!(pt.metrics.text_position, 1);
    let pt = layout.hit_test_point(Point::new(11.0, 0.0));
    assert_eq!(pt.metrics.text_position, 2);
}

#[test]
#[cfg(target_os = "macos")]
// for testing that 'middle' assignment in binary search is correct
fn test_hit_test_point_basic_1() {
    let mut text_layout = EmbedText::new();

    // base condition, one grapheme
    let font = text_layout
        .new_font_by_name("sans-serif", 12.0)
        .build()
        .unwrap();
    let layout = text_layout.new_text_layout(&font, "t").build().unwrap();
    println!("text pos 1: {:?}", layout.hit_test_text_position(1)); // 5.0

    // two graphemes (to check that middle moves)
    let pt = layout.hit_test_point(Point::new(1.0, 0.0));
    assert_eq!(pt.metrics.text_position, 0);

    let layout = text_layout.new_text_layout(&font, "te").build().unwrap();
    println!("text pos 1: {:?}", layout.hit_test_text_position(1)); // 5.0
    println!("text pos 2: {:?}", layout.hit_test_text_position(2)); // 12.0

    let pt = layout.hit_test_point(Point::new(1.0, 0.0));
    assert_eq!(pt.metrics.text_position, 0);
    let pt = layout.hit_test_point(Point::new(4.0, 0.0));
    assert_eq!(pt.metrics.text_position, 1);
    let pt = layout.hit_test_point(Point::new(6.0, 0.0));
    assert_eq!(pt.metrics.text_position, 1);
    let pt = layout.hit_test_point(Point::new(11.0, 0.0));
    assert_eq!(pt.metrics.text_position, 2);
}

#[test]
#[cfg(target_os = "linux")]
fn test_hit_test_point_complex_0() {
    // Notes on this input:
    // 6 code points
    // 7 utf-16 code units (1/1/1/1/1/2)
    // 14 utf-8 code units (2/1/3/3/1/4)
    // 4 graphemes
    let input = "é\u{0023}\u{FE0F}\u{20E3}1\u{1D407}"; // #️⃣,, 𝐇

    let mut text_layout = EmbedText::new();
    let font = text_layout
        .new_font_by_name("sans-serif", 12.0)
        .build()
        .unwrap();
    let layout = text_layout.new_text_layout(&font, input).build().unwrap();
    //println!("text pos 2: {:?}", layout.hit_test_text_position(2)); // 6.99999999
    //println!("text pos 9: {:?}", layout.hit_test_text_position(9)); // 24.0
    //println!("text pos 10: {:?}", layout.hit_test_text_position(10)); // 32.0
    //println!("text pos 14: {:?}", layout.hit_test_text_position(14)); // 39.0, line width

    let pt = layout.hit_test_point(Point::new(2.0, 0.0));
    assert_eq!(pt.metrics.text_position, 0);
    let pt = layout.hit_test_point(Point::new(4.0, 0.0));
    assert_eq!(pt.metrics.text_position, 2);
    let pt = layout.hit_test_point(Point::new(7.0, 0.0));
    assert_eq!(pt.metrics.text_position, 2);
    let pt = layout.hit_test_point(Point::new(10.0, 0.0));
    assert_eq!(pt.metrics.text_position, 2);
    let pt = layout.hit_test_point(Point::new(14.0, 0.0));
    assert_eq!(pt.metrics.text_position, 2);
    let pt = layout.hit_test_point(Point::new(18.0, 0.0));
    assert_eq!(pt.metrics.text_position, 9);
    let pt = layout.hit_test_point(Point::new(23.0, 0.0));
    assert_eq!(pt.metrics.text_position, 9);
    let pt = layout.hit_test_point(Point::new(26.0, 0.0));
    assert_eq!(pt.metrics.text_position, 9);
    let pt = layout.hit_test_point(Point::new(29.0, 0.0));
    assert_eq!(pt.metrics.text_position, 10);
    let pt = layout.hit_test_point(Point::new(32.0, 0.0));
    assert_eq!(pt.metrics.text_position, 10);
    let pt = layout.hit_test_point(Point::new(35.5, 0.0));
    assert_eq!(pt.metrics.text_position, 14);
    let pt = layout.hit_test_point(Point::new(38.0, 0.0));
    assert_eq!(pt.metrics.text_position, 14);
    let pt = layout.hit_test_point(Point::new(40.0, 0.0));
    assert_eq!(pt.metrics.text_position, 14);
}

#[test]
#[cfg(target_os = "macos")]
fn test_hit_test_point_complex_0() {
    // Notes on this input:
    // 6 code points
    // 7 utf-16 code units (1/1/1/1/1/2)
    // 14 utf-8 code units (2/1/3/3/1/4)
    // 4 graphemes
    let input = "é\u{0023}\u{FE0F}\u{20E3}1\u{1D407}"; // #️⃣,, 𝐇

    let mut text_layout = EmbedText::new();
    let font = text_layout
        .new_font_by_name("sans-serif", 12.0)
        .build()
        .unwrap();
    let layout = text_layout.new_text_layout(&font, input).build().unwrap();
    println!("text pos 2: {:?}", layout.hit_test_text_position(2)); // 6.673828125
    println!("text pos 9: {:?}", layout.hit_test_text_position(9)); // 28.55859375
    println!("text pos 10: {:?}", layout.hit_test_text_position(10)); // 35.232421875
    println!("text pos 14: {:?}", layout.hit_test_text_position(14)); // 42.8378905, line width

    let pt = layout.hit_test_point(Point::new(2.0, 0.0));
    assert_eq!(pt.metrics.text_position, 0);
    let pt = layout.hit_test_point(Point::new(4.0, 0.0));
    assert_eq!(pt.metrics.text_position, 2);
    let pt = layout.hit_test_point(Point::new(7.0, 0.0));
    assert_eq!(pt.metrics.text_position, 2);
    let pt = layout.hit_test_point(Point::new(10.0, 0.0));
    assert_eq!(pt.metrics.text_position, 2);
    let pt = layout.hit_test_point(Point::new(14.0, 0.0));
    assert_eq!(pt.metrics.text_position, 2);
    let pt = layout.hit_test_point(Point::new(18.0, 0.0));
    assert_eq!(pt.metrics.text_position, 9);
    let pt = layout.hit_test_point(Point::new(23.0, 0.0));
    assert_eq!(pt.metrics.text_position, 9);
    let pt = layout.hit_test_point(Point::new(26.0, 0.0));
    assert_eq!(pt.metrics.text_position, 9);
    let pt = layout.hit_test_point(Point::new(29.0, 0.0));
    assert_eq!(pt.metrics.text_position, 9);
    let pt = layout.hit_test_point(Point::new(32.0, 0.0));
    assert_eq!(pt.metrics.text_position, 10);
    let pt = layout.hit_test_point(Point::new(35.5, 0.0));
    assert_eq!(pt.metrics.text_position, 10);
    let pt = layout.hit_test_point(Point::new(38.0, 0.0));
    assert_eq!(pt.metrics.text_position, 10);
    let pt = layout.hit_test_point(Point::new(40.0, 0.0));
    assert_eq!(pt.metrics.text_position, 14);
    let pt = layout.hit_test_point(Point::new(43.0, 0.0));
    assert_eq!(pt.metrics.text_position, 14);
}

#[test]
#[cfg(target_os = "linux")]
fn test_hit_test_point_complex_1() {
    // this input caused an infinite loop in the binary search when test position
    // > 21.0 && < 28.0
    //
    // This corresponds to the char 'y' in the input.
    let input = "tßßypi";

    let mut text_layout = EmbedText::new();
    let font = text_layout
        .new_font_by_name("sans-serif", 12.0)
        .build()
        .unwrap();
    let layout = text_layout.new_text_layout(&font, input).build().unwrap();
    println!("text pos 0: {:?}", layout.hit_test_text_position(0)); // 0.0
    println!("text pos 1: {:?}", layout.hit_test_text_position(1)); // 5.0
    println!("text pos 2: {:?}", layout.hit_test_text_position(2)); // 5.0
    println!("text pos 3: {:?}", layout.hit_test_text_position(3)); // 13.0
    println!("text pos 4: {:?}", layout.hit_test_text_position(4)); // 13.0
    println!("text pos 5: {:?}", layout.hit_test_text_position(5)); // 21.0
    println!("text pos 6: {:?}", layout.hit_test_text_position(6)); // 28.0
    println!("text pos 7: {:?}", layout.hit_test_text_position(7)); // 36.0
    println!("text pos 8: {:?}", layout.hit_test_text_position(8)); // 39.0, end

    let pt = layout.hit_test_point(Point::new(27.0, 0.0));
    assert_eq!(pt.metrics.text_position, 6);
}

#[test]
#[cfg(target_os = "macos")]
fn test_hit_test_point_complex_1() {
    // this input caused an infinite loop in the binary search when test position
    // > 21.0 && < 28.0
    //
    // This corresponds to the char 'y' in the input.
    let input = "tßßypi";

    let mut text_layout = EmbedText::new();
    let font = text_layout
        .new_font_by_name("sans-serif", 12.0)
        .build()
        .unwrap();
    let layout = text_layout.new_text_layout(&font, input).build().unwrap();
    println!("text pos 0: {:?}", layout.hit_test_text_position(0)); // 0.0
    println!("text pos 1: {:?}", layout.hit_test_text_position(1)); // 5.0
    println!("text pos 2: {:?}", layout.hit_test_text_position(2)); // 5.0
    println!("text pos 3: {:?}", layout.hit_test_text_position(3)); // 13.0
    println!("text pos 4: {:?}", layout.hit_test_text_position(4)); // 13.0
    println!("text pos 5: {:?}", layout.hit_test_text_position(5)); // 21.0
    println!("text pos 6: {:?}", layout.hit_test_text_position(6)); // 28.0
    println!("text pos 7: {:?}", layout.hit_test_text_position(7)); // 36.0
    println!("text pos 8: {:?}", layout.hit_test_text_position(8)); // 39.0, end

    let pt = layout.hit_test_point(Point::new(27.0, 0.0));
    assert_eq!(pt.metrics.text_position, 6);
}