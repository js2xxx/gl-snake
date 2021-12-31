use bevy::prelude::*;
use rand::random;

pub struct Materials {
    head: Handle<ColorMaterial>,
    segment: Handle<ColorMaterial>,
    food: Handle<ColorMaterial>,
}

#[derive(Default)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct LastTailPosition(pub Option<crate::com::Position>);

#[derive(SystemLabel, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum SnakeState {
    Input,
    Movement,
    Eating,
    Growth,
}

pub struct GrowthEvent;

pub struct GameOverEvent;

pub fn setup(mut com: Commands, mut mat: ResMut<Assets<ColorMaterial>>) {
    com.spawn_bundle(OrthographicCameraBundle::new_2d());
    com.insert_resource(Materials {
        head: mat.add(Color::GRAY.into()),
        segment: mat.add(Color::DARK_GRAY.into()),
        food: mat.add(Color::FUCHSIA.into()),
    })
}

pub fn snake_spawner(
    mut com: Commands,
    mat: Res<Materials>,
    mut seg: ResMut<SnakeSegments>,
    pos: Query<&crate::com::Position>,
) {
    seg.0 = vec![
        com.spawn_bundle(SpriteBundle {
            material: mat.head.clone(),
            ..Default::default()
        })
        .insert(crate::com::SnakeHead {
            direction: crate::com::Direction::Up,
        })
        .insert(crate::com::SnakeSegment)
        .insert(crate::com::Position(5, 3))
        .insert(crate::com::Size::square(0.8))
        .id(),
        spawn_segment(&mut com, &mat.segment, crate::com::Position(5, 2)),
    ];
    food_spawner(com, mat, pos)
}

fn spawn_segment(
    com: &mut Commands,
    mat: &Handle<ColorMaterial>,
    pos: crate::com::Position,
) -> Entity {
    com.spawn_bundle(SpriteBundle {
        material: mat.clone(),
        ..Default::default()
    })
    .insert(crate::com::SnakeSegment)
    .insert(pos)
    .insert(crate::com::Size::square(0.65))
    .id()
}

pub fn input(key: Res<Input<KeyCode>>, mut heads: Query<&mut crate::com::SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir = if key.pressed(KeyCode::Left) | key.pressed(KeyCode::A) {
            crate::com::Direction::Left
        } else if key.pressed(KeyCode::Down) | key.pressed(KeyCode::S) {
            crate::com::Direction::Down
        } else if key.pressed(KeyCode::Up) | key.pressed(KeyCode::W) {
            crate::com::Direction::Up
        } else if key.pressed(KeyCode::Right) | key.pressed(KeyCode::D) {
            crate::com::Direction::Right
        } else {
            head.direction
        };

        if head.direction != dir.opposite() {
            head.direction = dir;
        }
    }
}

pub fn snake_movement(
    seg: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &crate::com::SnakeHead)>,
    mut pos: Query<&mut crate::com::Position>,
    mut last_tail_pos: ResMut<LastTailPosition>,
    mut game_over: EventWriter<GameOverEvent>,
) {
    if let Some((ent, head)) = heads.iter_mut().next() {
        let seg_pos = seg
            .0
            .iter()
            .map(|&ent| *pos.get_mut(ent).unwrap())
            .collect::<Vec<_>>();
        last_tail_pos.0 = seg_pos.last().copied();

        let head_pos = {
            let mut head_pos = pos.get_mut(ent).unwrap();
            let (x, y) = head.direction.delta();
            head_pos.0 += x;
            head_pos.1 += y;

            *head_pos
        };

        for (ent, new_pos) in seg.0.iter().skip(1).zip(seg_pos.into_iter()) {
            *pos.get_mut(*ent).unwrap() = new_pos;

            if new_pos == head_pos {
                game_over.send(GameOverEvent);
            }
        }

        use crate::com::ARENA;
        if head_pos.0 < 0
            || head_pos.1 < 0
            || head_pos.0 >= ARENA.0 as i32
            || head_pos.1 >= ARENA.1 as i32
        {
            game_over.send(GameOverEvent);
        }
    }
}

pub fn snake_eating(
    mut com: Commands,
    mut grow: EventWriter<GrowthEvent>,
    food_pos: Query<(Entity, &crate::com::Position), With<crate::com::Food>>,
    head_pos: Query<&crate::com::Position, With<crate::com::SnakeHead>>,
) {
    for head_pos in head_pos.iter() {
        for (ent, food_pos) in food_pos.iter() {
            if head_pos == food_pos {
                com.entity(ent).despawn();
                grow.send(GrowthEvent);
            }
        }
    }
}

pub fn snake_growth(
    mut com: Commands,
    last_tail_pos: Res<LastTailPosition>,
    mut seg: ResMut<SnakeSegments>,
    mut grow: EventReader<GrowthEvent>,
    mat: Res<Materials>,
    pos: Query<&crate::com::Position>,
) {
    if grow.iter().next().is_some() {
        seg.0.push(spawn_segment(
            &mut com,
            &mat.segment,
            last_tail_pos.0.unwrap(),
        ));
        food_spawner(com, mat, pos);
    }
}

pub fn game_over(
    mut com: Commands,
    mut game_over: EventReader<GameOverEvent>,
    mat: Res<Materials>,
    seg_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<crate::com::Food>>,
    seg: Query<Entity, With<crate::com::SnakeSegment>>,
    pos: Query<&crate::com::Position>
) {
    if game_over.iter().count() > 0 {
        for ent in food.iter().chain(seg.iter()) {
            com.entity(ent).despawn();
        }
        snake_spawner(com, mat, seg_res, pos);
    }
}

pub fn food_spawner(mut com: Commands, mat: Res<Materials>, pos: Query<&crate::com::Position>) {
    use crate::com::ARENA;

    let pos = pos.iter().copied().collect::<Vec<_>>();

    if pos.len() == (ARENA.0 * ARENA.1) as usize {
        return;
    }

    let new_pos = loop {
        let test = crate::com::Position(
            (random::<f32>() * ARENA.0 as f32) as i32,
            (random::<f32>() * ARENA.1 as f32) as i32,
        );
        if pos.iter().all(|pos| *pos != test) {
            break test;
        }
    };

    com.spawn_bundle(SpriteBundle {
        material: mat.food.clone(),
        ..Default::default()
    })
    .insert(crate::com::Food)
    .insert(new_pos)
    .insert(crate::com::Size::square(0.8));
}

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&crate::com::Size, &mut Sprite)>) {
    use crate::com::ARENA;
    let window = windows.get_primary().unwrap();
    for (size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            size.0.x / ARENA.0 as f32 * window.width() as f32,
            size.0.y / ARENA.1 as f32 * window.width() as f32,
        );
    }
}

pub fn position_translation(
    windows: Res<Windows>,
    mut q: Query<(&crate::com::Position, &mut Transform)>,
) {
    use crate::com::ARENA;

    fn convert(pos: f32, win: f32, arena: f32) -> f32 {
        let tile_size = win / arena;
        pos / arena * win - (win / 2.) + (tile_size / 2.)
    }

    let window = windows.get_primary().unwrap();
    for (pos, mut trans) in q.iter_mut() {
        trans.translation = Vec3::new(
            convert(pos.0 as f32, window.width() as f32, ARENA.0 as f32),
            convert(pos.1 as f32, window.height() as f32, ARENA.1 as f32),
            0.,
        )
    }
}
