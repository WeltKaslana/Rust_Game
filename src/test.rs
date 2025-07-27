use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::MouseWheel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_ui)
        .add_system(handle_scroll)
        .run();
}

#[derive(Component)]
struct ScrollableArea;

#[derive(Component)]
struct ScrollContent;

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // 创建主UI节点
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    }).with_children(|parent| {
        // 创建可滚动区域
        parent.spawn((NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                overflow: Overflow::Hidden,
                ..default()
            },
            border_color: Color::rgb(0.5, 0.5, 0.5).into(),
            background_color: Color::rgb(0.2, 0.2, 0.2).into(),
            ..default()
        }, ScrollableArea)).with_children(|parent| {
            // 创建滚动内容
            parent.spawn((NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(200.0), // 内容高度是容器的2倍，确保可以滚动
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                ..default()
            }, ScrollContent)).with_children(|parent| {
                // 添加一些内容元素用于滚动展示
                for i in 0..20 {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(5.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(
                            0.2 + (i as f32 * 0.03) % 0.7,
                            0.4 + (i as f32 * 0.02) % 0.5,
                            0.6 + (i as f32 * 0.01) % 0.3,
                        ).into(),
                        ..default()
                    }).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("Item {}", i + 1),
                            TextStyle {
                                font_size: 24.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    });
                }
            });
        });
    });
}

fn handle_scroll(
    mut scroll_events: EventReader<MouseWheel>,
    mut query: Query<(&mut Style, &Parent), With<ScrollContent>>,
    scrollable_query: Query<Entity, With<ScrollableArea>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_pos: Res<CursorPosition>,
) {
    let Ok(window) = window_query.get_single() else { return; };
    let Some(cursor_position) = mouse_pos.0 else { return; };
    
    // 检查鼠标是否在窗口内
    if cursor_position.x < 0.0 || cursor_position.x > window.width() ||
       cursor_position.y < 0.0 || cursor_position.y > window.height() {
        return;
    }
    
    for event in scroll_events.iter() {
        for (mut style, parent) in &mut query {
            // 确保我们只处理滚动区域内的内容
            if let Ok(scrollable_entity) = scrollable_query.get(parent.get()) {
                // 计算滚动量
                let scroll_amount = event.y * 20.0; // 调整滚动灵敏度
                
                // 应用滚动限制
                let max_scroll = 0.0;
                let min_scroll = -(style.height.percent - 100.0) * window.height() / 100.0;
                style.position.top = Val::Px((style.position.top.value() - scroll_amount).clamp(min_scroll, max_scroll));
            }
        }
    }
}

// 用于跟踪鼠标位置的资源
#[derive(Resource, Default)]
struct CursorPosition(Option<Vec2>);

// 系统：更新鼠标位置资源
fn update_cursor_position(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let Ok(window) = window_query.get_single() else { return; };
    
    if let Some(screen_position) = window.cursor_position() {
        cursor_position.0 = Some(Vec2::new(
            screen_position.x,
            window.height() - screen_position.y, // 转换为bevy坐标系（左下为原点）
        ));
    } else {
        cursor_position.0 = None;
    }
}   

