use std::{borrow::BorrowMut, collections::HashMap, error::Error, time::Instant};
use aspen_engine::AppBuilder;
use hecs::{Entity, With};
use rand::Rng;

struct RedTeam;
struct BlueTeam;
struct Health(u8);
struct Ammo(u8);
struct Bullet {
    pub target: Entity,
}
enum Winner {
    Red,
    Blue,
    Draw
}

struct PersistentData {
    pub update_count: usize,
    pub result: Option<Winner>,
    pub timestamps: HashMap<String, Instant>
}

fn main() -> Result<(), Box<dyn Error>> {
    let engine = AppBuilder::new(
        PersistentData { 
            update_count: 0,
            result: None,
            timestamps: HashMap::new()
        })?
        .use_graphics()?
        .add_update_func(|app| {
            if app.persistent.update_count == 0 {
                let mut rng = rand::thread_rng();
                let red_to_spawn = (0..5000).map(|_| {
                    (
                        Health(rng.gen_range(0..=100)), 
                        Ammo(rng.gen_range(50..=200)),
                        RedTeam
                    )
                });
                
                let mut rng = rand::thread_rng();
                let blue_to_spawn = (0..5000).map(|_| {
                    (
                        Health(rng.gen_range(0..=100)), 
                        Ammo(rng.gen_range(50..=200)), 
                        BlueTeam
                    )
                });
    
                app.ecs.spawn_batch(red_to_spawn);
                app.ecs.spawn_batch(blue_to_spawn);

                app.persistent.timestamps.insert("begin".to_string(), Instant::now());
            }

            app.persistent.update_count += 1;
        })
        .add_update_func(|app| {
            let mut bullets: Vec<(Bullet,)> = Vec::new();
            {
                let mut red = app.ecs.query::<With<(&mut Health, &mut Ammo), &RedTeam>>();
                let mut blue = app.ecs.query::<With<(&mut Health, &mut Ammo), &BlueTeam>>();
    
                let mut bi = blue.iter();
                for (_, (_, ammo)) in red.iter() {
                    if ammo.0 > 0 {
                        bullets.push((Bullet {
                            target: match bi.next() {
                                None => break,
                                Some(target) => target.0
                            }
                        },));
                        ammo.borrow_mut().0 -= 1;
                    } 
                }
                
                let mut ri = red.iter();
                for (_, (_, ammo)) in blue.iter() {
                    if ammo.0 > 0 {
                        bullets.push((Bullet {
                            target: match ri.next() {
                                None => break,
                                Some(target) => target.0
                            }
                        },));
                        ammo.borrow_mut().0 -= 1;
                    } 
                }
            }

            app.ecs.spawn_batch(bullets);
        })
        .add_update_func(|app| {
            let mut to_despawn = Vec::new();
            {
                let mut bullets = app.ecs.query::<&Bullet>();
                let bullets = bullets.iter();
    
    
                for (id, bullet) in bullets {
                    let mut health = app.ecs.get::<&mut Health>(bullet.target).unwrap();
                    if health.0 > 0 {
                        health.0 -= 1;
                    }
    
                    if health.0 == 0 {
                        to_despawn.push(bullet.target)
                    }
                    to_despawn.push(id)
                }
            }

            for id in to_despawn.iter() {
                _ = app.ecs.despawn(*id);
            }
        })
        .add_update_func(|app| {
            match app.persistent.result {
                None => {
                    let red_count;
                    let mut red_remaining_ammo: u32 = 0;
                    let blue_count;
                    let mut blue_remaining_ammo: u32 = 0;
        
                    {
                        let remaining_red = app.ecs.query_mut::<(&RedTeam, &Ammo)>();
                        let redi = remaining_red.into_iter();
                        red_count = redi.len();
                        for (_, (_, ammo)) in redi {
                            red_remaining_ammo += ammo.0 as u32
                        }
                    }
                    
                    {
                        let remaining_blue = app.ecs.query_mut::<(&BlueTeam, &Ammo)>();
                        let bluei = remaining_blue.into_iter();
                        blue_count = bluei.len();
                        for (_, (_, ammo)) in bluei {
                            blue_remaining_ammo += ammo.0 as u32
                        }
                    }

                    if red_count + blue_count == 0 {
                        app.persistent.result = Some(Winner::Draw);
                        println!("Draw because everybody was eliminated");
                    } else if red_count == 0 {
                        app.persistent.result = Some(Winner::Blue);
                        println!("Blue won by eliminating the red team");
                    } else if blue_count == 0 {
                        app.persistent.result = Some(Winner::Red);
                        println!("Red won by eliminating the blue team");
                    } else if red_remaining_ammo + blue_remaining_ammo == 0 {
                        app.persistent.result = Some(Winner::Draw);
                        println!("Draw due to running out of ammo");
                    } else if red_remaining_ammo == 0 {
                        app.persistent.result = Some(Winner::Blue);
                        println!("Blue won because red ran out of ammo");
                    } else if blue_remaining_ammo == 0 {
                        app.persistent.result = Some(Winner::Red);
                        println!("Red won because blue ran out of ammo");
                    } else {
                        return
                    }

                    println!(
                        "finished after {}ms and {} cycles", 
                        app.persistent.timestamps.get("begin").unwrap().elapsed().as_millis(),
                        app.persistent.update_count
                    );
                    app.exit()
                },
                Some(_) => ()
            }
        })
        .build()?;

    engine.run()?;

    Ok(())
}