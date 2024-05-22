use macroquad::math::f32;
use macroquad::prelude::*;

// Rectangle is AABB with vector, width and height
type Rect = (f32, f32, f32, f32);
type MinkowskiSum1 = [Vec2; 8];
type MinkowskiSum2 = [Vec2; 32];

#[macroquad::main("BasicShapes")]
async fn main() {
  request_new_screen_size(1800.0, 1200.0);

  let speed = 100.0;

  let (mut cx, mut cy) = (900.0, 500.0);

  let (mut x1, mut y1, w1, h1) = (100.0, 100.0, 75.0, 100.0);
  let (mut x2, mut y2, w2, h2) = (400.0, 300.0, 100.0, 100.0);

  // let triangle_translation = vec2(400.0, 400.0);
  // let triangle_translation = vec2(-400.0, 400.0);
  let translation_vector = vec2(400.0, -400.0);
  // let triangle_translation = vec2(-400.0, -400.0);

  // let triangle = [vec2(0.0, 0.0), vec2(100.0, 0.0), vec2(50.0, 100.0)];
  let triangle = [vec2(0.0, 0.0) + translation_vector, vec2(100.0, 0.0) + translation_vector, vec2(50.0, 100.0) + translation_vector];

  // x,y,w,h and use the translation_vector for x,y
  let rectangle = (100.0 + translation_vector.x, 100.0 + translation_vector.y, 100.0, 100.0);

  loop {
    let dt = get_frame_time();
    clear_background(LIGHTGRAY);

    // World origin is moved with WASD keys
    if is_key_down(KeyCode::Up) { cy -= 200.0 * dt; }
    if is_key_down(KeyCode::Down) { cy += 200.0 * dt; }
    if is_key_down(KeyCode::Left) { cx -= 200.0 * dt; }
    if is_key_down(KeyCode::Right) { cx += 200.0 * dt; }

    // First rectangle is moved with arrow keys without SHIFT
    if is_key_down(KeyCode::W) && !is_key_down(KeyCode::LeftShift) { y1 -= 200.0 * dt; }
    if is_key_down(KeyCode::S) && !is_key_down(KeyCode::LeftShift) { y1 += 200.0 * dt; }
    if is_key_down(KeyCode::A) && !is_key_down(KeyCode::LeftShift) { x1 -= 200.0 * dt; }
    if is_key_down(KeyCode::D) && !is_key_down(KeyCode::LeftShift) { x1 += 200.0 * dt; }

    // Second rectangle is moved with arrow keys with SHIFT
    if is_key_down(KeyCode::W) && is_key_down(KeyCode::LeftShift) { y2 -= 200.0 * dt; }
    if is_key_down(KeyCode::S) && is_key_down(KeyCode::LeftShift) { y2 += 200.0 * dt; }
    if is_key_down(KeyCode::A) && is_key_down(KeyCode::LeftShift) { x2 -= 200.0 * dt; }
    if is_key_down(KeyCode::D) && is_key_down(KeyCode::LeftShift) { x2 += 200.0 * dt; }

    let (mouse_x, mouse_y) = mouse_position();
    let mouse_position = vec2(mouse_x, mouse_y);

    let world_mouse_position = world_to_screen_coordinates(mouse_position, (cx, cy));

    // from world origin to world mouse
    // let line = world_mouse_position;

    let direction = world_mouse_position.normalize();
    let opposite_direction = world_mouse_position.normalize();
    let rotated_direction = vec2(-opposite_direction.y, opposite_direction.x);

    // let line = speed * direction;
    let line = world_mouse_position;

    // print the screen position of the mouse
    draw_text(&format!("Mouse: ({:.2}, {:.2})", mouse_x, mouse_y), 10.0, 20.0, 20.0, BLACK);
    draw_text(&format!("World: ({:.2}, {:.2})", world_mouse_position.x, world_mouse_position.y), 10.0, 40.0, 20.0, BLACK);

    draw_translated_rectangle((x1, y1, w1, h1), BLUE, (cx, cy));
    draw_translated_rectangle((x2, y2, w2, h2), RED, (cx, cy));

    // Create MinkowskiSum1 and with the line added to the rectangle
    let minkowski_sum1 = add_line_to_rect(line, (x1, y1, w1, h1));

    draw_minowski_sum1(minkowski_sum1, (cx, cy));

    // Create MinkowskiSum2 with the rectangle subtracted from the MinkowskiSum1
    let minkowski_sum2 = subtract_rect_to_minkowski_sum1((x2, y2, w2, h2), minkowski_sum1, (cx, cy));


    draw_minkowski_sum2_largest_four_bounding_boxes(minkowski_sum2, (cx, cy), line);
    draw_minkowski_sum2_redux(minkowski_sum2, (cx, cy));

    // draw the world_mouse_position
    draw_translated_line(vec2(0.0, 0.0), line, 2.0, BLACK, (cx, cy));

    // draw the triangle in black or red if the mouse is inside
    if is_point_in_triangle(world_mouse_position, triangle) {
      draw_translated_triangle(triangle, RED, (cx, cy));
    } else {
      draw_translated_triangle(triangle, BLACK, (cx, cy));
    }

    // draw the rectangle in black or red if the mouse is inside
    if is_point_in_aabb(world_mouse_position, rectangle) {
      draw_translated_rectangle(rectangle, RED, (cx, cy));
    } else {
      draw_translated_rectangle(rectangle, BLACK, (cx, cy));
    }

    // TODO: DELETE
    // draw text in red if is_swept_aabb_on_aabb is true
    /*if is_swept_aabb_on_aabb(1.0, line, 1.0, (x1, y1, w1, h1), (x2, y2, w2, h2)) {
      // print the arguments of the function

      draw_text("Swept AABB on AABB", 10.0, 160.0, 20.0, RED);
    }*/

    // draw translate lines to mark origin with cross
    draw_translated_line(vec2(-10.0, 0.0), vec2(10.0, 0.0), 2.0, BLACK, (cx, cy));
    draw_translated_line(vec2(0.0, -10.0), vec2(0.0, 10.0), 2.0, BLACK, (cx, cy));

    // TODO TODO CLEAN UP //

    // let (new_coordinates, _) = move_aabb_to_direction(speed, direction, dt, (x1, y1, w1, h1), (x2, y2, w2, h2));
    let (new_coordinates, remaining_time, collision_normal) = move_aabb_to_direction(1.0, line, 1.0, (x1, y1, w1, h1), (x2, y2, w2, h2), (cx, cy));

    // draw the first rectangle with the new coordinates
    let new_rect = (new_coordinates.x, new_coordinates.y, w1, h1);

    // new heading vector from line and flipped based on collision normal
    let mut heading = line.normalize();

    if collision_normal.x != 0.0 {
      heading.x = -heading.x;
    }

    if collision_normal.y != 0.0 {
      heading.y = -heading.y;
    }

    // fraction of "line" based on the "leftover" values are used for line2
    let line2 = line.length() * heading * remaining_time;

    let (new_coordinates, remaining_time, collision_normal) = move_aabb_to_direction(1.0, line2, 1.0, new_rect, (x2, y2, w2, h2), (cx, cy));
    let new_rect = (new_coordinates.x, new_coordinates.y, w1, h1);

    draw_translated_rectangle(new_rect, DARKPURPLE, (cx, cy));

    next_frame().await
  }
}

// Function that adds a vector to a rectangle using Minkowski sum and returns 8 new vectors
// The vector is drawn form origin to the vector's end; ignore the origin as it's just 0,0
fn add_line_to_rect(line: Vec2, rect: Rect) -> MinkowskiSum1 {
  let (x, y, w, h) = rect;

  let corners = [
    vec2(x, y),
    vec2(x + w, y),
    vec2(x + w, y + h),
    vec2(x, y + h),
  ];

  [
    // origin (0,0) "added" the four corners
    corners[0],
    corners[1],
    corners[2],
    corners[3],

    // line (the end coordinates) added to the four corners
    corners[0] + line,
    corners[1] + line,
    corners[2] + line,
    corners[3] + line,
  ]
}

fn subtract_rect_to_minkowski_sum1(rect: Rect, minkowski_sum: MinkowskiSum1, offset: (f32, f32)) -> MinkowskiSum2 {
  // subtraction is the same as addition, but the
  // rectangle should be made into opposite itself in relation to origin

  let (x, y, w, h) = rect;
  let corners = [
    vec2(x, y),
    vec2(x + w, y),
    vec2(x + w, y + h),
    vec2(x, y + h),
  ];

  let opposite_corners = [
    vec2(-x, -y),
    vec2(-x - w, -y),
    vec2(-x - w, -y - h),
    vec2(-x, -y - h),
  ];

  // draw circles at the corners of the rectangle
  /*for i in 0..4 {
    // draw_circle(corners[i].x, corners[i].y, 3.0, PURPLE);
    draw_translated_circle(corners[i], 3.0, PURPLE, offset);
  }*/

  // draw circles at the opposite corners of the rectangle
  /*for i in 0..4 {
    // draw_circle(opposite_corners[i].x, opposite_corners[i].y, 3.0, DARKPURPLE);
    draw_translated_circle(opposite_corners[i], 3.0, DARKPURPLE, offset);
  }*/

  [
    // first opposite_corner against all minkowski_sum's vectors
    opposite_corners[0] + minkowski_sum[0],
    opposite_corners[0] + minkowski_sum[1],
    opposite_corners[0] + minkowski_sum[2],
    opposite_corners[0] + minkowski_sum[3],
    opposite_corners[0] + minkowski_sum[4],
    opposite_corners[0] + minkowski_sum[5],
    opposite_corners[0] + minkowski_sum[6],
    opposite_corners[0] + minkowski_sum[7],

    // second opposite_corner against all minkowski_sum's vectors
    opposite_corners[1] + minkowski_sum[0],
    opposite_corners[1] + minkowski_sum[1],
    opposite_corners[1] + minkowski_sum[2],
    opposite_corners[1] + minkowski_sum[3],
    opposite_corners[1] + minkowski_sum[4],
    opposite_corners[1] + minkowski_sum[5],
    opposite_corners[1] + minkowski_sum[6],
    opposite_corners[1] + minkowski_sum[7],

    // third opposite_corner against all minkowski_sum's vectors
    opposite_corners[2] + minkowski_sum[0],
    opposite_corners[2] + minkowski_sum[1],
    opposite_corners[2] + minkowski_sum[2],
    opposite_corners[2] + minkowski_sum[3],
    opposite_corners[2] + minkowski_sum[4],
    opposite_corners[2] + minkowski_sum[5],
    opposite_corners[2] + minkowski_sum[6],
    opposite_corners[2] + minkowski_sum[7],

    // fourth opposite_corner against all minkowski_sum's vectors
    opposite_corners[3] + minkowski_sum[0],
    opposite_corners[3] + minkowski_sum[1],
    opposite_corners[3] + minkowski_sum[2],
    opposite_corners[3] + minkowski_sum[3],
    opposite_corners[3] + minkowski_sum[4],
    opposite_corners[3] + minkowski_sum[5],
    opposite_corners[3] + minkowski_sum[6],
    opposite_corners[3] + minkowski_sum[7],
  ]
}

fn draw_translated_rectangle((x, y, w, h): Rect, color: Color, offset: (f32, f32)) {
  draw_rectangle(x + offset.0, y + offset.1, w, h, color);
}

fn draw_translated_circle(pos: Vec2, radius: f32, color: Color, offset: (f32, f32)) {
  draw_circle(pos.x + offset.0, pos.y + offset.1, radius, color);
}

fn draw_translated_line(start: Vec2, end: Vec2, thickness: f32, color: Color, offset: (f32, f32)) {
  draw_line(start.x + offset.0, start.y + offset.1, end.x + offset.0, end.y + offset.1, thickness, color);
}

fn draw_minowski_sum1(minkowski_sum: MinkowskiSum1, offset: (f32, f32)) {
  for i in 0..8 {
    for j in 0..8 {
      draw_translated_line(minkowski_sum[i], minkowski_sum[j], 2.0, GREEN, offset);
    }
  }
}

fn draw_minowski_sum1_redux(minkowski_sum: MinkowskiSum1, offset: (f32, f32)) {
  // List of indexes to skip drawing lines between
  let skip = [0, 1, 3, 4, 5, 7];

  for i in 0..8 {
    for j in 0..8 {
      if !(skip.contains(&i) || skip.contains(&j)) {
        draw_translated_line(minkowski_sum[i], minkowski_sum[j], 2.0, GREEN, offset);
      }
    }
  }
}

fn draw_minkowski_sum2(minkowski_sum: MinkowskiSum2, offset: (f32, f32)) {
  for i in 0..32 {
    for j in 0..32 {
      draw_translated_line(minkowski_sum[i], minkowski_sum[j], 2.0, PURPLE, offset);
    }
  }
}

fn draw_minkowski_sum2_redux(minkowski_sum: MinkowskiSum2, offset: (f32, f32)) {
  // List of indexes to skip drawing lines between
  let skip = [0, 1, 3, 4, 5, 7, 8, 9, 10, 12, 13, 14, 17, 18, 19, 21, 22, 23, 24, 26, 27, 28, 30, 31];

  // Useful indexes
  // 2, 11, 16, 25
  // 6, 15, 20, 29

  for i in 0..32 {
    for j in 0..32 {
      if !(skip.contains(&i) || skip.contains(&j)) {
        draw_translated_line(minkowski_sum[i], minkowski_sum[j], 2.0, PURPLE, offset);

        // small red circles also
        draw_translated_circle(minkowski_sum[i], 5.0, RED, offset);
      }
    }
  }
}

fn screen_to_world_coordinates(screen: Vec2, offset: (f32, f32)) -> Vec2 {
  vec2(screen.x + offset.0, screen.y + offset.1)
}

fn world_to_screen_coordinates(world: Vec2, offset: (f32, f32)) -> Vec2 {
  vec2(world.x - offset.0, world.y - offset.1)
}

fn is_point_in_aabb(point: Vec2, (x, y, w, h): Rect) -> bool {
  point.x > x && point.x < x + w && point.y > y && point.y < y + h
}

fn is_point_in_triangle(point: Vec2, triangle: [Vec2; 3]) -> bool {
  let (a, b, c) = (triangle[0], triangle[1], triangle[2]);

  let area = 0.5 * (-b.y * c.x + a.y * (-b.x + c.x) + a.x * (b.y - c.y) + b.x * c.y);
  let s = 1.0 / (2.0 * area) * (a.y * c.x - a.x * c.y + (c.y - a.y) * point.x + (a.x - c.x) * point.y);
  let t = 1.0 / (2.0 * area) * (a.x * b.y - a.y * b.x + (a.y - b.y) * point.x + (b.x - a.x) * point.y);

  s > 0.0 && t > 0.0 && 1.0 - s - t > 0.0
}

fn is_rectangle_in_rectangle(rect1: Rect, rect2: Rect) -> bool {
  let (x1, y1, w1, h1) = rect1;
  let (x2, y2, w2, h2) = rect2;

  // Based on top-left corners and width and height

  // if the first rectangle is left of the second rectangle
  // or the first rectangle is right of the second rectangle
  // or the first rectangle is above the second rectangle
  // or the first rectangle is below the second rectangle
  // then the rectangles are not colliding

  if x1 + w1 < x2 || x1 > x2 + w2 || y1 + h1 < y2 || y1 > y2 + h2 {
    return false;
  }

  return true;
}

// Two rectangles are passed, return both x and y edge-to-edge distances
fn xy_distances(rect1: Rect, rect2: Rect) -> (f32, f32) {
  let (x1, y1, w1, h1) = rect1;
  let (x2, y2, w2, h2) = rect2;

  let mut x_distance = 0.0;
  let mut y_distance = 0.0;


  // left vs right
  if x1 + w1 < x2 {
    x_distance = x2 - (x1 + w1);
  }

  // right vs left
  if x1 > x2 + w2 {
    x_distance = x2 + w2 - x1;
  }

  // top vs bottom
  if y1 + h1 < y2 {
    y_distance = y2 - (y1 + h1);
  }

  // bottom vs top
  if y1 > y2 + h2 {
    y_distance = y2 + h2 - y1;
  }

  (x_distance, y_distance)
}

fn draw_translated_triangle(triangle: [Vec2; 3], color: Color, offset: (f32, f32)) {
  let (a, b, c) = (triangle[0], triangle[1], triangle[2]);
  let offset_vec = vec2(offset.0, offset.1);
  // draw_triangle(a.x + offset.0, a.y + offset.1, b.x + offset.0, b.y + offset.1, c.x + offset.0, c.y + offset.1, color);

  // pub fn draw_triangle(v1: Vec2, v2: Vec2, v3: Vec2, color: Color) {
  draw_triangle(a + offset_vec, b + offset_vec, c + offset_vec, color);
}

fn draw_minkowski_sum2_largest_four_bounding_boxes(minkowski_sum: MinkowskiSum2, offset: (f32, f32), heading: Vec2) {
  // Useful indexes
  // 2, 11, 16, 25
  // 6, 15, 20, 29

  // The vectors in the data are points in a wireframe structure.
  // Some of the vectors are the corners but the sum calculations can have 4 different forms where the set of outermost vectors are different
  // The largest four bounding boxes involve the outtermost vectors and drawing rectangles with the vectors representing the corners
  //
  // Two opposite vectors are only part of the wireframe, 2 others can be combined from x and y of the opposite vectors

  // Opposite vectors for rectangles
  // NW expansion with corners of SW: 2; NW: 20
  // NE expansion with corners of SW: 11; NE: 29
  // SE expansion with corners of NE: 16; SE: 6
  // SW expansion with corners of NE: 25; SW: 15

  // Additionally two triangles are drawn with two other vectors and 3rd corner being their "x & y" intersection
  // NW expansion SW triangle with vectors: 11, 15 and NE triangle with 25, 29
  // NE expansion SE triangle with vectors: 2, 6 and NW triangle with 16, 20
  // SE expansion SW triangle with vectors: 15, 11 and NE triangle with 29, 25
  // SW expansion SE triangle with vectors: 6, 2 and NW triangle with 20, 16

  // Create bounding box for NW case

  let zero = vec2(0.0, 0.0);

  // TODO DELETE
  // let nw_bb = (minkowski_sum[2].x, minkowski_sum[2].y, minkowski_sum[2].x - minkowski_sum[20].x, minkowski_sum[2].y - minkowski_sum[20].y);

  let nw_bb = (minkowski_sum[20].x, minkowski_sum[20].y, minkowski_sum[2].x - minkowski_sum[20].x, minkowski_sum[2].y - minkowski_sum[20].y);
  let nw_triangle_sw = [minkowski_sum[11], minkowski_sum[15], vec2(minkowski_sum[15].x, minkowski_sum[11].y)];
  let nw_triangle_ne = [minkowski_sum[29], minkowski_sum[25], vec2(minkowski_sum[25].x, minkowski_sum[29].y)];

  // See what the heading is and draw if it's NW, NE, SE or SW

  if heading.x <= 0.0 && heading.y <= 0.0 {
    //*
    draw_translated_rectangle(nw_bb, RED, offset);
    draw_translated_triangle(nw_triangle_sw, GREEN, offset);
    draw_translated_triangle(nw_triangle_ne, BLUE, offset);
    // */

    // Is the origin in the NW bounding box but not in the SW or NE triangles?
    if is_point_in_aabb(zero, nw_bb) && !is_point_in_triangle(zero, nw_triangle_sw) && !is_point_in_triangle(zero, nw_triangle_ne) {
      draw_text("NW", 10.0, 80.0, 20.0, BLACK);
    }
  }

  // 29 is at the top right and 11 is at the bottom left
  // calculate top-left x,y,w,h
  let ne_bb = (minkowski_sum[11].x, minkowski_sum[29].y, minkowski_sum[29].x - minkowski_sum[11].x, minkowski_sum[11].y - minkowski_sum[29].y);

  let ne_triangle_se = [minkowski_sum[2], minkowski_sum[6], vec2(minkowski_sum[6].x, minkowski_sum[2].y)];
  let ne_triangle_nw = [minkowski_sum[16], minkowski_sum[20], vec2(minkowski_sum[16].x, minkowski_sum[20].y)];


  if heading.x > 0.0 && heading.y <= 0.0 {
    //*
    draw_translated_rectangle(ne_bb, RED, offset);
    draw_translated_triangle(ne_triangle_se, GREEN, offset);
    draw_translated_triangle(ne_triangle_nw, BLUE, offset);
    // */

    // Is the origin in the NE bounding box but not in the SE or NW triangles?
    if is_point_in_aabb(zero, ne_bb) && !is_point_in_triangle(zero, ne_triangle_se) && !is_point_in_triangle(zero, ne_triangle_nw) {
      draw_text("NE", 10.0, 100.0, 20.0, BLACK);
    }
  }

  let se_bb = (minkowski_sum[16].x, minkowski_sum[16].y, minkowski_sum[6].x - minkowski_sum[16].x, minkowski_sum[6].y - minkowski_sum[16].y);
  let se_triangle_sw = [minkowski_sum[15], minkowski_sum[11], vec2(minkowski_sum[11].x, minkowski_sum[15].y)];
  let se_triangle_ne = [minkowski_sum[29], minkowski_sum[25], vec2(minkowski_sum[29].x, minkowski_sum[25].y)];

  if heading.x > 0.0 && heading.y > 0.0 {
    //*
    draw_translated_rectangle(se_bb, RED, offset);
    draw_translated_triangle(se_triangle_sw, GREEN, offset);
    draw_translated_triangle(se_triangle_ne, BLUE, offset);
    // */

    // Is the origin in the SE bounding box but not in the SW or NE triangles?
    if is_point_in_aabb(zero, se_bb) && !is_point_in_triangle(zero, se_triangle_sw) && !is_point_in_triangle(zero, se_triangle_ne) {
      draw_text("SE", 10.0, 120.0, 20.0, BLACK);
    }
  }

  // 25 is top-right and 15 is bottom-left
  // calculate top-left x,y,w,h
  let sw_bb = (minkowski_sum[15].x, minkowski_sum[25].y, minkowski_sum[25].x - minkowski_sum[15].x, minkowski_sum[15].y - minkowski_sum[25].y);

  let sw_triangle_se = [minkowski_sum[6], minkowski_sum[2], vec2(minkowski_sum[2].x, minkowski_sum[6].y)];
  let sw_triangle_nw = [minkowski_sum[20], minkowski_sum[16], vec2(minkowski_sum[20].x, minkowski_sum[16].y)];

  if heading.x <= 0.0 && heading.y > 0.0 {
    //*
    draw_translated_rectangle(sw_bb, RED, offset);
    draw_translated_triangle(sw_triangle_se, GREEN, offset);
    draw_translated_triangle(sw_triangle_nw, BLUE, offset);
    // */

    // Is the origin in the SW bounding box but not in the SE or NW triangles?
    if is_point_in_aabb(zero, sw_bb) && !is_point_in_triangle(zero, sw_triangle_se) && !is_point_in_triangle(zero, sw_triangle_nw) {
      draw_text("SW", 10.0, 140.0, 20.0, BLACK);
    }
  }
}

fn is_swept_aabb_on_aabb(movement: Vec2, rect: Rect, rect2: Rect) -> bool {
  let (x1, y1, w1, h1) = rect;
  let (x2, y2, w2, h2) = rect2;

  let direction = movement.normalize();
  let zero = vec2(0.0, 0.0);

  let corners_0 = vec2(x1, y1);
  let corners_1 = vec2(x1 + w1, y1);
  let corners_2 = vec2(x1 + w1, y1 + h1);
  let corners_3 = vec2(x1, y1 + h1);

  let minkowski_sum1_0 = corners_0; // + 0,0
  let minkowski_sum1_1 = corners_1; // + 0,0
  let minkowski_sum1_2 = corners_2; // + 0,0
  let minkowski_sum1_3 = corners_3; // + 0,0

  let minkowski_sum1_4 = corners_0 + movement;
  let minkowski_sum1_5 = corners_1 + movement;
  let minkowski_sum1_6 = corners_2 + movement;
  let minkowski_sum1_7 = corners_3 + movement;

  let negated_corners_0 = vec2(-x2, -y2);
  let negated_corners_1 = vec2(-x2 - w2, -y2);
  let negated_corners_2 = vec2(-x2 - w2, -y2 - h2);
  let negated_corners_3 = vec2(-x2, -y2 - h2);

  let minkowski_sum2_2  = negated_corners_0 + minkowski_sum1_2;
  let minkowski_sum2_6  = negated_corners_0 + minkowski_sum1_6;
  let minkowski_sum2_11 = negated_corners_1 + minkowski_sum1_3;
  let minkowski_sum2_15 = negated_corners_1 + minkowski_sum1_7;
  let minkowski_sum2_16 = negated_corners_2 + minkowski_sum1_0;
  let minkowski_sum2_20 = negated_corners_2 + minkowski_sum1_4;
  let minkowski_sum2_25 = negated_corners_3 + minkowski_sum1_1;
  let minkowski_sum2_29 = negated_corners_3 + minkowski_sum1_5;

  // Calculate the areas that are tested against the zero vector

  let nw_bb = (minkowski_sum2_20.x, minkowski_sum2_20.y, minkowski_sum2_2.x - minkowski_sum2_20.x, minkowski_sum2_2.y - minkowski_sum2_20.y);
  let ne_bb = (minkowski_sum2_11.x, minkowski_sum2_29.y, minkowski_sum2_29.x - minkowski_sum2_11.x, minkowski_sum2_11.y - minkowski_sum2_29.y);
  let se_bb = (minkowski_sum2_16.x, minkowski_sum2_16.y, minkowski_sum2_6.x - minkowski_sum2_16.x, minkowski_sum2_6.y - minkowski_sum2_16.y);
  let sw_bb = (minkowski_sum2_15.x, minkowski_sum2_25.y, minkowski_sum2_25.x - minkowski_sum2_15.x, minkowski_sum2_15.y - minkowski_sum2_25.y);

  let nw_triangle_sw = [minkowski_sum2_11, minkowski_sum2_15, vec2(minkowski_sum2_15.x, minkowski_sum2_11.y)];
  let nw_triangle_ne = [minkowski_sum2_29, minkowski_sum2_25, vec2(minkowski_sum2_25.x, minkowski_sum2_29.y)];

  let ne_triangle_se = [minkowski_sum2_2, minkowski_sum2_6, vec2(minkowski_sum2_6.x, minkowski_sum2_2.y)];
  let ne_triangle_nw = [minkowski_sum2_16, minkowski_sum2_20, vec2(minkowski_sum2_16.x, minkowski_sum2_20.y)];

  let se_triangle_sw = [minkowski_sum2_15, minkowski_sum2_11, vec2(minkowski_sum2_11.x, minkowski_sum2_15.y)];
  let se_triangle_ne = [minkowski_sum2_29, minkowski_sum2_25, vec2(minkowski_sum2_29.x, minkowski_sum2_25.y)];

  let sw_triangle_se = [minkowski_sum2_6, minkowski_sum2_2, vec2(minkowski_sum2_2.x, minkowski_sum2_6.y)];
  let sw_triangle_nw = [minkowski_sum2_20, minkowski_sum2_16, vec2(minkowski_sum2_20.x, minkowski_sum2_16.y)];

  if direction.x <= 0.0 && direction.y <= 0.0 && is_point_in_aabb(zero, nw_bb) && !is_point_in_triangle(zero, nw_triangle_sw) && !is_point_in_triangle(zero, nw_triangle_ne) {
    return true;
  }

  if direction.x > 0.0 && direction.y <= 0.0 && is_point_in_aabb(zero, ne_bb) && !is_point_in_triangle(zero, ne_triangle_se) && !is_point_in_triangle(zero, ne_triangle_nw) {
    return true;
  }

  if direction.x > 0.0 && direction.y > 0.0 && is_point_in_aabb(zero, se_bb) && !is_point_in_triangle(zero, se_triangle_sw) && !is_point_in_triangle(zero, se_triangle_ne) {
    return true;
  }

  if direction.x <= 0.0 && direction.y > 0.0 && is_point_in_aabb(zero, sw_bb) && !is_point_in_triangle(zero, sw_triangle_se) && !is_point_in_triangle(zero, sw_triangle_nw) {
    return true;
  }

  false
}

// move a rectangle as far as you can in a direction and return the remaining time
// use the same arguments as is_swep_aabb_on_aabb
// wrap is_swept_aabb_on_aabb and work out if two rectangles are colliding
// move the first rectangle to the tested direction
// if collision happens work out the point of collision
// return a tuple with new top-left coordinates, remaining time and collision normal
fn move_aabb_to_direction(speed: f32, direction: Vec2, dt: f32, rect: Rect, rect2: Rect, offset: (f32, f32)) -> (Vec2, f32, Vec2) {
  let movement = speed * direction * dt;
  let collided = is_swept_aabb_on_aabb(movement, rect, rect2);

  if !collided {
    let position = vec2(rect.0, rect.1);
    let delta_movement = speed * direction * dt;
    let new_position = position + delta_movement;

    draw_translated_line(position + vec2(0.5 * rect.2, 0.5 * rect.3), new_position + vec2(0.5 * rect.2, 0.5 * rect.3), 2.0, BLACK, offset);

    return (new_position, 0.0, vec2(0.0, 0.0));
  }

  let vx = direction.x * speed;
  let vy = direction.y * speed;

  let (x_distance, y_distance) = xy_distances(rect, rect2);


  let movement_time: f32;
  let collision_normal: Vec2;

  let x_time = x_distance / vx;
  let y_time = y_distance / vy;

  if vx == 0.0 {

    movement_time = y_time;
    collision_normal = vec2(0.0, -vy.signum());

  } else if vy == 0.0 {

    movement_time = x_time;
    collision_normal = vec2(-vx.signum(), 0.0);

  } else {
    movement_time = x_time.max(y_time);

    if x_time < y_time {
      collision_normal = vec2(0.0, -vy.signum());
    } else {
      collision_normal = vec2(-vx.signum(), 0.0);
    }

  }

  draw_text(&format!("speed: {:.4}, vy: {:.4}", vx, vy), 10.0, 180.0, 20.0, RED);
  draw_text(&format!("distance: {:.4}, dy: {:.4}", x_distance, y_distance), 10.0, 200.0, 20.0, RED);
  draw_text(&format!("time: {:.4}, ty: {:.4}", x_time, y_time), 10.0, 220.0, 20.0, RED);
  draw_text(&format!("movement_time: {:.4}", movement_time), 10.0, 240.0, 20.0, RED);

  let remaining_time = dt - movement_time;

  let position = vec2(rect.0, rect.1);
  let delta_movement = speed * direction * movement_time;
  let new_position = position + delta_movement;

  draw_translated_line(position + vec2(0.5 * rect.2, 0.5 * rect.3), new_position + vec2(0.5 * rect.2, 0.5 * rect.3), 2.0, BLACK, offset);

  (new_position, remaining_time, collision_normal)
}

/*
// AABB Sweep using 1D analysis
// Assuming that AABB's can't collide unless they have 1D overlaps in x and y at the same time
// These can be called as "enter" and "exit" moments
// Collisions are 2 "enter" moments before any "exit" moments

// Thinking the possible cases as a 3x3 grid
// Corners need to have two 1D overlaps at the same time to
// Non-corners start with 1D overlap and need to have the other 1D
// Return new top-left coordinates for the first rectangle (the one that moves)
*/
// TODO: TOO HARD
/*fn sweep2(movement: Vec2, rect1: Rect, rect2: Rect) /*-> Vec*/ {
  let (x1, y1, w1, h1) = rect1;
  let (x2, y2, w2, h2) = rect2;

  // top, bottom, left, right walls for rect1 and rect2
  // reminder: Rect's have coordinate in top-left corner
  let top1 = y1;
  let bottom1 = y1 + h1;
  let left1 = x1;
  let right1 = x1 + w1;

  let top2 = y2;
  let bottom2 = y2 + h2;
  let left2 = x2;
  let right2 = x2 + w2;

  // Do not deal with "existing collision" case
  let (x_distance, y_distance) = xy_distances(rect1, rect2); // TODO, lose the xy_distances function
  if x_distance == 0.0 || y_distance == 0.0 { return; }

  // Do not deal with "not approaching" cases

  // Rect1 is fully on the left and not moving right
  if right1 <= left2 && movement.x <= 0.0 { return; }

  // Rect1 is fully on the right and not moving left
  if left1 >= right2 && movement.x >= 0.0 { return; }

  // Rect1 is fully on the top and not moving down
  if bottom1 <= top2 && movement.y <= 0.0 { return; }

  // Rect1 is fully on the bottom and not moving up
  if top1 >= bottom2 && movement.y >= 0.0 { return; }

  // We are either approaching from outside or
  // We have x,y movement while in above or side with the other rectangle and might not "miss" the other rectangle

  let mut x_entry = 0.0;

  if x_distance != 0.0 {
    x_entry = x_distance / movement.x;
  }

  let mut x_exit  = 0.0;



  let mut y_entry = 0.0;

  if y_distance != 0.0 {
    y_entry = y_distance / movement.y;
  }


  let mut y_exit  = 0.0;


}*/

/*fn is_same_sign(a: f32, b: f32) -> bool {
  a * b > 0.0
}*/